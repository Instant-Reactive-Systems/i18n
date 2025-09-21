use fluent_syntax::ast::Entry;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitBool, LitStr, Token};
use unic_langid::LanguageIdentifier;

struct LoadMacroInput {
    path: LitStr,
    fallback_lang: LitStr,
    check_keys: bool,
    name: Ident,
}

impl Parse for LoadMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "Usage: load!(\"i18n\", fallback_lang = \"en-US\")\nThe path should be relative to your crate root (where Cargo.toml is)."));
        }

        let path: LitStr = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected a path to the locales directory as the first argument. The path should be relative to your crate root (where Cargo.toml is).")
        })?;

        input
            .parse::<Token![,]>()
            .map_err(|_| syn::Error::new(input.span(), "Expected a comma after the path."))?;

        let ident: Ident = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "Expected `fallback_lang = \"...\"` after the comma.",
            )
        })?;

        if ident != "fallback_lang" {
            return Err(syn::Error::new(
                ident.span(),
                "Expected the identifier `fallback_lang`.",
            ));
        }

        input
            .parse::<Token![=]>()
            .map_err(|_| syn::Error::new(input.span(), "Expected `=` after `fallback_lang`."))?;

        let fallback_lang: LitStr = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "Expected a string literal for the fallback language.",
            )
        })?;

        let mut check_keys = true;
        let mut name = Ident::new("LOCALES", Span::call_site());
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "check_keys" => check_keys = input.parse::<LitBool>()?.value(),
                "name" => name = input.parse::<Ident>()?,
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        "Unexpected parameter, expected 'check_keys' or 'name'",
                    ))
                }
            }
        }

        Ok(LoadMacroInput {
            path,
            fallback_lang,
            check_keys,
            name,
        })
    }
}

pub fn load_impl(input: TokenStream) -> TokenStream {
    let LoadMacroInput {
        path: path_lit,
        fallback_lang,
        check_keys,
        name,
    } = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    // Verify the fallback language identifier at compile time.
    if let Err(err) = fallback_lang.value().parse::<LanguageIdentifier>() {
        return syn::Error::new(
            fallback_lang.span(),
            format!("Invalid fallback language identifier: {}", err),
        )
        .to_compile_error()
        .into();
    }

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

    let fallback_lang_str = fallback_lang.value();

    quote! {
        i18n::lazy_static::lazy_static! {
            pub static ref #name: i18n::Locales = {
                let fallback_lang = #fallback_lang_str.parse().expect("Invalid fallback language identifier");
                let mut locales = i18n::Locales::new(fallback_lang);
                #(#add_locale)*
                locales
            };
        }
    }
    .into()
}
