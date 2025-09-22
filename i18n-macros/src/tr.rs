use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, LitStr, Token};

struct TrMacroInput {
    lang: Expr,
    id: LitStr,
    locales_var: Ident,
    main_args: Vec<(Ident, Expr)>,
    attr_args: HashMap<String, Vec<(Ident, Expr)>>,
    on_error: Option<Expr>,
}

impl Parse for TrMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lang: Expr = input.parse().map_err(|err| {
            syn::Error::new(
                err.span(),
                "Expected language argument (e.g., `langid!(\"en-US\")` or a variable).",
            )
        })?;
        input.parse::<Token![,]>().map_err(|err| {
            syn::Error::new(err.span(), "Expected a comma after the language argument.")
        })?;
        let id: LitStr = input
            .parse()
            .map_err(|err| syn::Error::new(err.span(), "Expected message ID (string literal)."))?;

        let mut locales_var = Ident::new("LOCALES", Span::call_site());
        let mut main_args = Vec::new();
        let mut attr_args: HashMap<String, Vec<(Ident, Expr)>> = HashMap::new();
        let mut on_error = None;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            // Check for 'locales = VAR_NAME' or 'on_error = EXPR' or message arguments
            if input.peek(Ident) && input.peek2(Token![=]) {
                let key_ident: Ident = input.parse()?;
                input.parse::<Token![=]>()?;

                if key_ident == "locales" {
                    locales_var = input.parse()?;
                } else if key_ident == "on_error" {
                    on_error = Some(input.parse()?);
                } else {
                    // This is a main message arg: key = value
                    // The key_ident has already been parsed, so we just need the value
                    let value: Expr = input.parse()?;
                    main_args.push((key_ident, value));
                }
            } else if input.peek(Ident) && input.peek2(syn::token::Paren) {
                // Check for attr(...)
                // This is an attribute arg: attr(<attr-id>, <variable-name> = <value>)
                let _attr_ident: Ident = input.parse()?; // Parse 'attr'
                let content;
                syn::parenthesized!(content in input); // Parse content within parentheses

                let attr_id: LitStr = content.parse()?;
                content.parse::<Token![,]>()?;
                let arg_key: Ident = content.parse()?;
                content.parse::<Token![=]>()?;
                let arg_value: Expr = content.parse()?;

                attr_args
                    .entry(attr_id.value())
                    .or_default()
                    .push((arg_key, arg_value));
            } else {
                return Err(input.error(
                    "Unexpected token. Expected `locales = VAR_NAME`, `on_error = EXPR`, `attr(...)`, or `key = value`."
                ));
            }
        }

        Ok(TrMacroInput {
            lang,
            id,
            locales_var,
            main_args,
            attr_args,
            on_error,
        })
    }
}

pub fn tr_impl(input: TokenStream) -> TokenStream {
    let TrMacroInput {
        lang,
        id,
        locales_var,
        on_error,
        main_args,
        attr_args,
    } = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut query_builder = quote! { i18n_loader::Query::new(#id) };

    for (key, value) in main_args.into_iter() {
        query_builder = quote! { #query_builder.with_arg(stringify!(#key), #value) };
    }
    for (attr_name, args) in attr_args.into_iter() {
        for (key, value) in args {
            query_builder =
                quote! { #query_builder.with_attr_arg(#attr_name, stringify!(#key), #value) };
        }
    }

    let query_call = quote! {
        #locales_var.query(&#lang, &#query_builder)
    };

    let final_expansion = if let Some(on_error_expr) = on_error {
        quote! {
            match #query_call {
                Ok(msg) => msg,
                Err(errs) => #on_error_expr(errs),
            }
        }
    } else {
        quote! {
            match #query_call {
                Ok(msg) => msg,
                Err(errs) => {
                    i18n_loader::Message {
                        id: #id.to_string(),
                        value: #id.to_string(),
                        attrs: std::collections::HashMap::new(),
                    }
                }
            }
        }
    };

    TokenStream::from(final_expansion)
}
