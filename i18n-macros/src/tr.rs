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
    main_args: Vec<(String, Expr)>,
    attr_args: HashMap<String, Vec<(String, Expr)>>,
}

impl Parse for TrMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lang: Expr = input.parse().map_err(|err| {
            syn::Error::new(
                err.span(),
                "Expected a language identifier (e.g., `langid!(\"en-US\")` or a variable).",
            )
        })?;
        input.parse::<Token![,]>().map_err(|err| {
            syn::Error::new(
                err.span(),
                "Expected a comma after the language identifier.",
            )
        })?;
        let id: LitStr = input.parse().map_err(|err| {
            syn::Error::new(err.span(), "Expected a message ID (a string literal).")
        })?;

        let mut locales_var = Ident::new("LOCALES", Span::call_site());
        let mut main_args = Vec::new();
        let mut attr_args: HashMap<String, Vec<(String, Expr)>> = HashMap::new();

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            if input.peek(Ident) && input.peek2(Token![=]) {
                let key_ident: Ident = input.parse()?;
                input.parse::<Token![=]>()?;

                if key_ident == "locales" {
                    locales_var = input.parse()?;
                } else {
                    return Err(
                        input.error("Unexpected identifier. Expected `locales = VAR_NAME`.")
                    );
                }
            } else if input.peek(LitStr) && input.peek2(syn::token::Paren) {
                // This is a main message arg: key = value
                let key: LitStr = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: Expr = input.parse()?;
                main_args.push((key.value(), value));
            } else if input.peek(Ident) && input.peek2(syn::token::Paren) {
                // Check for attr(...)
                // This is an attribute arg: attr(<attr-id>, <variable-name> = <value>)
                let attr_ident: Ident = input.parse()?;
                if attr_ident != "attr" {
                    return Err(input.error("Unexpected identifier. Expected `attr(...)`"));
                }

                let content;
                syn::parenthesized!(content in input);

                let attr_id: LitStr = content.parse()?;
                content.parse::<Token![,]>()?;
                let arg_key: LitStr = content.parse()?;
                content.parse::<Token![=]>()?;
                let arg_value: Expr = content.parse()?;

                attr_args
                    .entry(attr_id.value())
                    .or_default()
                    .push((arg_key.value(), arg_value));
            } else {
                return Err(input.error(
                    "Unexpected token. Expected `locales = VAR_NAME`, `attr(...)`, or `key = value`."
                ));
            }
        }

        Ok(TrMacroInput {
            lang,
            id,
            locales_var,
            main_args,
            attr_args,
        })
    }
}

pub fn tr_impl(input: TokenStream) -> TokenStream {
    let TrMacroInput {
        lang,
        id,
        locales_var,
        main_args,
        attr_args,
    } = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut query_builder = quote! { i18n::Query::new(#id) };

    for (key, value) in main_args.into_iter() {
        query_builder = quote! { #query_builder.with_arg(stringify!(#key), #value) };
    }
    for (attr_name, args) in attr_args.into_iter() {
        for (key, value) in args {
            query_builder = quote! { #query_builder.with_attr_arg(#attr_name, #key, #value) };
        }
    }

    let query_call = quote! {
        #locales_var.query(&#lang, &#query_builder)
    };

    let final_expansion = quote! {
        match #query_call {
            Ok(msg) => msg,
            Err(_err) => {
                i18n::Message {
                    id: #id.to_string(),
                    value: #id.to_string(),
                    attrs: Default::default(),
                }
            }
        }
    };

    TokenStream::from(final_expansion)
}
