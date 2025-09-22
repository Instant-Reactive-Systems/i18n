mod langid_to_country_flag;
mod langid_to_dir;
mod langid_to_name;

use self::langid_to_country_flag::*;
use self::langid_to_dir::*;
use self::langid_to_name::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

/// Extracts all used languages from the given locale path.
pub fn langs_impl(input: TokenStream) -> TokenStream {
    let usage = "Usage: langs!(\"i18n\")\nThe path should be relative to your crate root (where Cargo.toml is).";
    if input.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), usage)
            .to_compile_error()
            .into();
    }

    let input_path: LitStr = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => {
            let msg = "Expected a path to the locales directory, relative to your crate root (where Cargo.toml is).";
            return syn::Error::new(err.span(), msg).to_compile_error().into();
        }
    };
    let path = input_path.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut absolute_path = std::path::PathBuf::from(manifest_dir);
    absolute_path.push(&path);

    // Read directories in the specified path
    let langs = std::fs::read_dir(&absolute_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // Check if it's a directory
            if !path.is_dir() {
                return None;
            }

            // Extract language ID from directory name
            let dir_name = path.file_name()?.to_str()?.to_string();
            let splitter = if dir_name.contains('_') {
                "_".to_string()
            } else if dir_name.contains('-') {
                "-".to_string()
            } else {
                dir_name.clone()
            };
            let mut parts = dir_name.split(&splitter);
            let langid = parts
                .next()
                .map(str::to_lowercase)
                .expect("should always be present");
            let region = parts.next().map(str::to_uppercase);
            let full_langid = if let Some(region) = &region {
                format!("{}-{}", langid, region)
            } else {
                langid.clone()
            };
            let name = langid_to_name(&langid);
            let flag = region.map(|region| langid_to_flag(&region));
            let dir = langid_to_dir(&langid);

            Some(quote! {
                i18n::Lang {
                    id: #full_langid,
                    name: #name,
                    flag: #flag,
                    dir: #dir,
                }
            })
        })
        .collect::<Vec<_>>();

    // Generate the token stream representing the array of Lang instances
    let expanded = quote! {
        [#(#langs),*]
    };

    TokenStream::from(expanded)
}
