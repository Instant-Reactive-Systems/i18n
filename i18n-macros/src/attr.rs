use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, Ident, LitStr, Token,
};

// Simplified representation of arguments
type Args = HashMap<String, Expr>;

// A helper function to parse the optional arguments, including `locales`.
fn parse_optional_args(input: ParseStream) -> Result<(Args, Ident)> {
    let mut args = Args::new();
    let mut locales_var = None;

    while !input.is_empty() {
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            break; // Allow trailing comma
        }

        // Check for `locales = ...` which is keyed by an Ident
        if input.peek(Ident) && input.peek2(Token![=]) {
            let key: Ident = input.parse()?;
            if key == "locales" {
                input.parse::<Token![=]>()?;
                locales_var = Some(input.parse()?);
                continue;
            } else {
                return Err(syn::Error::new(key.span(), "Unexpected identifier. Only `locales` is supported as a keyword argument."));
            }
        }

        // Otherwise, parse a Fluent argument, which is keyed by a LitStr
        let key: LitStr = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;
        args.insert(key.value(), value);
    }
    
    let locales = locales_var.unwrap_or_else(|| Ident::new("LOCALES", Span::call_site()));
    Ok((args, locales))
}

pub struct AttrMacroInput {
    from: Expr,
    attr: LitStr,
    args: Args,
    locales: Ident,
}

impl Parse for AttrMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let from: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let attr: LitStr = input.parse()?;

        let (args, locales) = parse_optional_args(input)?;

        Ok(AttrMacroInput { from, attr, args, locales })
    }
}

pub fn attr_impl(input: TokenStream) -> TokenStream {
    let AttrMacroInput { from, attr, args, locales } = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let (args_creation, args_variable) = if args.is_empty() {
        (quote! {}, quote! { None })
    } else {
        let mut stmts = quote! { let mut fluent_args = i18n::FluentArgs::new(); };
        for (key, value) in args.into_iter() {
            stmts.extend(quote! { fluent_args.set(#key, i18n::FluentValue::from(#value)); });
        }
        (stmts, quote! { Some(&fluent_args) })
    };

    let final_expansion = quote! {
        {
            #args_creation
            let args = #args_variable;
            let query_result = match #from.attrs.get_mut(#attr) {
                Some(attr_cache) => attr_cache.query(args),
                None => Err(vec![i18n::FluentError::ResolverError(
                    i18n::ResolverError::Reference(i18n::ReferenceKind::Message {
                        id: #from.id.clone(),
                        attribute: Some(#attr.to_string()),
                    }),
                )]),
            };
            match query_result {
                Ok(s) => s,
                Err(errs) => {
                    #locales.call_on_error(&errs);
                    #attr.to_string()
                }
            }
        }
    };

    TokenStream::from(final_expansion)
}