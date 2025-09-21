mod langs;
mod load;
mod tr;

use proc_macro::TokenStream;

/// Extracts language information from a specified directory.
///
/// This macro reads the subdirectories of the given path, treating each subdirectory
/// as a language ID. It then generates a `[i18n::Lang; ...]` array containing
/// metadata for each found language (ID, name, flag, direction).
///
/// The path should be relative to your crate root (where Cargo.toml is).
///
/// # Usage
///
/// ```rust
/// use i18n_macros::langs;
///
/// let available_langs = langs!("./i18n");
/// // available_langs will be an array like:
/// // [
/// //   i18n::Lang { id: "en-US", name: "English", flag: Some("🇺🇸"), dir: "ltr" },
/// //   i18n::Lang { id: "hr-HR", name: "Croatian", flag: Some("🇭🇷"), dir: "ltr" },
/// // ]
/// ```
#[proc_macro]
pub fn langs(input: TokenStream) -> TokenStream {
    langs::langs_impl(input)
}

/// Loads Fluent localization files from a specified directory and creates a
/// lazily-initialized static instance of `i18n_loader::Locales`.
///
/// This macro reads `.ftl` files from subdirectories of the given path (each
/// subdirectory representing a locale). It parses these files at compile time
/// and embeds their content into your binary.
///
/// The generated static instance provides methods to query for localized messages.
///
/// # Parameters
///
/// - `path`: A string literal representing the path to the locales directory.
///   This path should be relative to your crate root (where Cargo.toml is).
/// - `fallback_lang`: A string literal representing the language identifier
///   (e.g., "en-US") to use as a fallback if a message is not found in the
///   requested language. This parameter is required.
/// - `check_keys` (optional): A boolean literal (`true` or `false`). If `true`
///   (default), the macro will perform a compile-time check to ensure all
///   locale files have a consistent set of message keys. If `false`, this
///   check is skipped.
/// - `name` (optional): An identifier to use as the name for the generated
///   `lazy_static` variable. Defaults to `LOCALES`.
///
/// # Usage
///
/// ```rust
/// use i18n_macros::load;
/// use i18n_loader::{Query, LanguageIdentifier};
/// use unic_langid::langid;
///
/// // Basic usage with default name (LOCALES) and key checking
/// load!("./i18n", fallback_lang = "en-US");
///
/// // With a custom name and disabled key checking
/// load!("./i18n", fallback_lang = "en-US", check_keys = false, name = MY_I18N_DATA);
///
/// // Example of accessing the generated data (assuming default name LOCALES)
/// // let lang = langid!("en-US");
/// // let query = Query::new("welcome-message");
/// // let message = LOCALES.query(&lang, &query).unwrap();
/// // println!("{}", message.value);
/// ```
#[proc_macro]
pub fn load(input: TokenStream) -> TokenStream {
    load::load_impl(input)
}

/// Queries for a localized message, returning an `i18n_loader::Message` instance.
///
/// This macro provides a convenient way to query for a message using a language
/// identifier, message ID, and optional arguments. It handles error cases by
/// either calling a custom error handler or providing a default fallback message.
///
/// # Syntax
///
/// `tr!(lang: Expr, id: LitStr [, locales = VAR_NAME] [, key = value]* [, .attribute_name(key = value)* ] [, on_error = EXPR])`
///
/// - `lang`: A Rust expression that evaluates to a `&LanguageIdentifier` (e.g., `langid!("en-US")` or a variable). This is the language to query for.
/// - `id`: A string literal representing the ID of the Fluent message.
/// - `locales` (optional): An identifier for the `i18n_loader::Locales` static variable to use. Defaults to `LOCALES`.
/// - `key = value`: Optional key-value pairs for arguments to the main message.
///   `key` must be an identifier, and `value` can be any Rust expression.
/// - `attr(attr_id, key = value)`: Optional arguments for a specific attribute
///   of the message. `attr_id` is a string literal representing the attribute ID (e.g., "aria-label" or "attr-arg").
///   `key` must be an identifier, and `value` can be any Rust expression.
/// - `on_error` (optional): A Rust expression (e.g., a closure or function call) that will be executed if the localization query fails.
///   It must accept one argument of type `Vec<i18n_loader::FluentError>` and return an `i18n_loader::Message`.
///   If not provided, a default `i18n_loader::Message` is returned (with the ID as its value) and errors are printed to `eprintln!`.
///
/// # Returns
///
/// An `i18n_loader::Message` instance.
///
/// # Examples
///
/// ```rust
/// use i18n_macros::tr;
/// use i18n_loader::{Message, FluentError};
/// use unic_langid::langid;
///
/// // Assuming `LOCALES` is loaded via `load!` macro
/// // load!("./i18n", fallback_lang = "en-US");
///
/// let lang_en = langid!("en-US");
///
/// // Basic usage
/// let msg1 = tr!(lang_en, "welcome-message");
///
/// // With main message arguments
/// let msg2 = tr!(lang_en, "greeting", user = "Alice", count = 5);
///
/// // With attribute arguments
/// let msg3 = tr!(lang_en, "login-button", attr("aria-label", user = "Bob"));
/// let msg4 = tr!(lang_en, "login-button", attr("attr-arg", user = "Bob"));
///
/// // With custom locales variable
/// // load!("./i18n", fallback_lang = "en-US", name = MY_APP_LOCALES);
/// // let msg5 = tr!(lang_en, "app-title", locales = MY_APP_LOCALES);
///
/// // With custom error handling
/// fn handle_tr_error(errs: Vec<FluentError>) -> Message {
///     eprintln!("Custom error handler: {{:?}}", errs);
///     Message { id: "error".to_string(), value: "Error occurred!".to_string(), attrs: Default::default() }
/// }
/// let msg6 = tr!(lang_en, "non-existent-id", on_error = handle_tr_error);
///
/// // With both main message and attribute arguments
/// let msg7 = tr!(lang_en, "item-status", item = "book", status = "available", attr("tooltip", id = 123));
/// ```
#[proc_macro]
pub fn tr(input: TokenStream) -> TokenStream {
    tr::tr_impl(input)
}
