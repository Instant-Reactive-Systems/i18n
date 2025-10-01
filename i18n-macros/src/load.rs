use fluent_syntax::ast::Entry;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, LitBool, LitStr, Token};
use unic_langid::LanguageIdentifier;

struct LoadMacroInput {
    path: LitStr,
    fallback_lang: Option<LitStr>,
    check_keys: bool,
    name: Ident,
    on_error: Option<Expr>,
}

impl Parse for LoadMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Usage: load!(\"i18n\")\nOptional parameters: `fallback_lang`, `check_keys`, `name`, `on_error`.\nThe path should be relative to your crate root (where Cargo.toml is).",
            ));
        }

        let path: LitStr = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected a path to the locales directory as the first argument. The path should be relative to your crate root (where Cargo.toml is).")
        })?;

        let mut fallback_lang = None;
        let mut check_keys = true;
        let mut name = Ident::new("LOCALES", Span::call_site());
        let mut on_error = None;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "fallback_lang" => fallback_lang = Some(input.parse()?),
                "check_keys" => check_keys = input.parse::<LitBool>()?.value(),
                "name" => name = input.parse::<Ident>()?,
                "on_error" => on_error = Some(input.parse::<Expr>()?),
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        "Unexpected parameter, expected 'fallback_lang', 'check_keys', 'name', or 'on_error'",
                    ))
                }
            }
        }

        Ok(LoadMacroInput {
            path,
            fallback_lang,
            check_keys,
            name,
            on_error,
        })
    }
}

pub fn load_impl(input: TokenStream) -> TokenStream {
    let LoadMacroInput {
        path: path_lit,
        fallback_lang,
        check_keys,
        name,
        on_error,
    } = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let fallback_lang = match fallback_lang {
        Some(lang) => {
            // Verify the fallback language identifier at compile time.
            if let Err(err) = lang.value().parse::<LanguageIdentifier>() {
                return syn::Error::new(
                    lang.span(),
                    format!("Invalid fallback language identifier: {}", err),
                )
                .to_compile_error()
                .into();
            }
            let lang_str = lang.value();
            quote! { #lang_str }
        }
        None => quote! { "en-US" },
    };

    let path = path_lit.value();
    let path = Path::new(&path);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut absolute_path = std::path::PathBuf::from(manifest_dir);
    absolute_path.push(&path);

    let entries = match std::fs::read_dir(&absolute_path) {
        Ok(entries) => entries,
        Err(err) => {
            return syn::Error::new(
                path_lit.span(),
                format!("Expected directory '{path:?}' ({absolute_path:?}): {err}"),
            )
            .to_compile_error()
            .into()
        }
    };

    let mut errors = Vec::new();
    let mut locale_contents: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_keys: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
    let mut all_absolute_file_paths: Vec<String> = Vec::default();

    for entry in entries {
        let Ok(entry) = entry else { continue };
        if !entry.path().is_dir() {
            continue;
        }

        let locale = entry.file_name().to_string_lossy().to_string();
        let locale_path = entry.path();
        let Ok(files) = std::fs::read_dir(&locale_path) else {
            continue;
        };
        for file in files {
            let Ok(file) = file else { continue };
            let file_path = file.path();
            if file_path.extension().and_then(|ext| ext.to_str()) != Some("ftl") {
                continue;
            }

            // track the file using `include_str!`
            if let Some(path) = file_path.to_str() {
                all_absolute_file_paths.push(path.to_string());
            }

            let file_name = file.file_name().to_string_lossy().to_string();
            let content = match std::fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(err) => {
                    errors.push(format!("Failed to read {locale}/{file_name}: {err}"));
                    continue;
                }
            };

            let resource = match fluent_syntax::parser::parse(content.as_str()) {
                Ok(resource) => resource,
                Err((_, errs)) => {
                    let msgs = errs.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>();
                    errors.push(format!(
                        "Failed to parse {locale}/{file_name}: {}",
                        msgs.join("; ")
                    ));
                    continue;
                }
            };

            let mut keys = HashSet::new();
            for entry in resource.body.iter() {
                match entry {
                    Entry::Message(msg) => _ = keys.insert(msg.id.name.to_string()),
                    Entry::Term(term) => _ = keys.insert(term.id.name.to_string()),
                    _ => {}
                }
            }

            file_keys
                .entry(file_name.clone())
                .or_default()
                .insert(locale.clone(), keys);
            locale_contents
                .entry(locale.clone())
                .or_default()
                .push(content);
        }
    }

    if check_keys {
        for (file_name, locale_keysets) in &file_keys {
            let all_keys: HashSet<String> = locale_keysets
                .values()
                .flat_map(|s| s.iter())
                .cloned()
                .collect();
            for (locale, keys) in locale_keysets {
                let missing: Vec<String> = all_keys
                    .iter()
                    .filter(|k| !keys.contains(*k))
                    .cloned()
                    .collect();
                if !missing.is_empty() {
                    errors.push(format!(
                        "Missing keys in {locale}/{file_name}: {}",
                        missing.join(", ")
                    ));
                }
            }
        }
    }

    if !errors.is_empty() {
        let err_quotes = errors.iter().map(|msg| quote! { compile_error!(#msg); });
        return quote! { #(#err_quotes)* }.into();
    }

    let locales: Vec<String> = locale_contents.keys().cloned().collect();
    let add_locale = locales.into_iter().map(|locale| {
        let contents = locale_contents.get(&locale).unwrap();
        let create_fluent_resources = contents.into_iter().map(|content| quote! {
                i18n::FluentResource::try_new(#content.to_string()).expect("parsed at compile time")
            }).collect::<Vec<_>>();

        quote! {
            locales.add_locale(#locale, vec![ #(#create_fluent_resources),* ]);
        }
    });

    let on_error = on_error.map_or_else(|| quote! { None }, |expr| quote! { Some(#expr) });

    let trackers = all_absolute_file_paths.iter().enumerate().map(|(i, path)| {
        let const_name = quote::format_ident!("_I18N_TRACKER_{}", i);
        quote! {
            // This const is never used, but it makes the compiler track changes to the file.
            #[allow(dead_code)]
            const #const_name: &str = include_str!(#path);
        }
    });

    quote! {
        i18n::lazy_static::lazy_static! {
            pub static ref #name: i18n::Locales = {
                #(#trackers)*
                let mut locales = i18n::Locales::new(#fallback_lang.parse().expect("compile time verified"), #on_error);
                #(#add_locale)*
                locales
            };
        }
    }
    .into()
}
