mod attr;
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
/// ```ignore
/// use i18n_macros::langs;
///
/// let available_langs = langs!("../tests/i18n");
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
/// lazily-initialized static instance of `i18n::Locales`.
///
/// This macro reads `.ftl` files from subdirectories of the given path (each
/// subdirectory representing a locale). It parses these files at compile time
/// and embeds their content into your binary.
///
/// The generated static instance provides methods to query for localized messages.
///
/// # Syntax
///
/// `load!(path: LitStr [, fallback_lang: LitStr] [, check_keys: bool] [, name: Ident] [, on_error: Expr])`
///
/// # Arguments
///
/// - `path`: A string literal representing the path to the locales directory.
///   This path should be relative to your crate root (where `Cargo.toml` is).
///
/// - `fallback_lang` (optional): A string literal representing the language identifier
///   (e.g., "en-US") to use as a fallback if a message is not found in the
///   requested language. Defaults to `"en-US"`.
///
/// - `check_keys` (optional): A boolean literal (`true` or `false`). If `true`
///   (default), the macro will perform a compile-time check to ensure all
///   locale files have a consistent set of message keys. If `false`, this
///   check is skipped.
///
/// - `name` (optional): An identifier to use as the name for the generated
///   `lazy_static` variable. Defaults to `LOCALES`.
///
/// - `on_error` (optional): An expression that evaluates to a function or closure
///   to be called when an error occurs during localization (e.g., missing message).
///   The function should have the signature `fn(errors: &[i18n::FluentError])`.
///
/// # Usage
///
/// ```ignore
/// // Basic usage with default values.
/// i18n::load!("../tests/i18n", name = LOCALES_DEFAULT);
///
/// // With a custom fallback language, disabled key checking, and a custom name.
/// i18n::load!(
///     "../tests/i18n",
///     fallback_lang = "hr-hr",
///     check_keys = false,
///     name = MY_I18N_DATA
/// );
///
/// // With an error handler.
/// fn on_error(errors: &[i18n::FluentError]) {
///     // Log the error, send it to a monitoring service, etc.
///     println!("Localization errors: {:?}", errors);
/// }
///
/// i18n::load!("../tests/i18n", on_error = on_error, name = LOCALES_WITH_ERROR_HANDLER);
///
/// // Example of accessing the generated data (assuming default name `LOCALES_DEFAULT`).
/// let lang_en = i18n::langid!("en-US");
/// let query = i18n::Query::new("foo-a");
/// let message = LOCALES_DEFAULT.query(&lang_en, &query).unwrap();
/// assert_eq!(message.value, "English A".to_string());
///
/// let lang_hr = i18n::langid!("hr-hr");
/// let query = i18n::Query::new("foo-a");
/// let message = LOCALES_DEFAULT.query(&lang_hr, &query).unwrap();
/// assert_eq!(message.value, "Croatian A".to_string());
/// ```
#[proc_macro]
pub fn load(input: TokenStream) -> TokenStream {
    load::load_impl(input)
}

/// Queries for a localized message, returning an `i18n::Message` instance.
///
/// This macro provides a convenient way to query for a message using a language
/// identifier, message ID, and optional arguments. It handles error cases by
/// providing a default fallback message.
///
/// # Syntax
///
/// `tr!(lang: Expr, id: LitStr [, locales = VAR_NAME] [, key = value]* [, .attribute_name(key = value)* ])`
///
/// - `lang`: A Rust expression that evaluates to a `&LanguageIdentifier` (e.g., `langid!("en-US")` or a variable). This is the language to query for.
/// - `id`: A string literal representing the ID of the Fluent message.
/// - `locales` (optional): An identifier for the `i18n::Locales` static variable to use. Defaults to `LOCALES`.
/// - `key = value`: Optional key-value pairs for arguments to the main message.
///   `key` must be an string literal, and `value` can be any Rust expression.
/// - `attr(attr_id, key = value)`: Optional arguments for a specific attribute
///   of the message. `attr_id` is a string literal representing the attribute ID (e.g., "aria-label" or "attr-arg").
///
/// # Returns
///
/// An `i18n::Message` instance.
///
/// # Examples
///
/// ```ignore
/// // Load the localization data.
/// i18n::load!("../tests/i18n", fallback_lang = "en-US", name = TR_LOCALES);
///
/// let lang_en = i18n::langid!("en-US");
///
/// // Basic usage:
/// let msg1 = i18n::tr!(lang_en, "foo-a", locales = TR_LOCALES);
/// assert_eq!(msg1.value, "English A".to_string());
///
/// // With main message arguments:
/// let msg2 = i18n::tr!(lang_en, "welcome-back", "username" = "Alice", locales = TR_LOCALES);
/// assert_eq!(msg2.value, "Welcome back, \u{2068}Alice\u{2069}!".to_string());
///
/// // With attribute arguments:
/// let msg3 = i18n::tr!(lang_en, "login-btn", attr("attr-arg", "text" = "some text"), locales = TR_LOCALES);
/// assert_eq!(msg3.attrs.get("attr-arg"), Some(&"This is an attribute argument with arbitrary text: \u{2068}some text\u{2069}".to_string()));
///
/// // With a custom locales variable:
/// i18n::load!("../tests/i18n", fallback_lang = "en-US", name = MY_APP_LOCALES);
/// let msg4 = i18n::tr!(lang_en, "foo-b", locales = MY_APP_LOCALES);
/// assert_eq!(msg4.value, "English B".to_string());
/// ```
#[proc_macro]
pub fn tr(input: TokenStream) -> TokenStream {
    tr::tr_impl(input)
}

/// Queries an attribute from an existing `i18n::Message` instance.
///
/// This macro provides a convenient way to lazily query for an attribute's
/// value from a `Message` struct that you already have. If the query fails,
/// it will invoke the `on_error` handler on the `Locales` instance.
///
/// # Syntax
///
/// `attr!(message_expr, attr_id: LitStr [, locales = VAR_NAME] [, key = value]*)`
///
/// - `message_expr`: An expression that evaluates to a `&mut i18n::Message`.
/// - `attr_id`: A string literal representing the ID of the attribute to query.
/// - `locales` (optional): The `i18n::Locales` instance to use for the error callback. Defaults to `LOCALES`.
/// - `key = value`: Optional key-value pairs for arguments to the attribute.
///
/// # Returns
///
/// A `String`. If the attribute doesn't exist or an error occurs during formatting,
/// it returns the attribute ID as a fallback string.
///
/// # Example
///
/// ```ignore
/// i18n::load!("../tests/i18n", name = ATTR_MACRO_LOCALES);
/// let lang = i18n::langid!("en-US");
///
/// let mut msg = i18n::tr!(lang, "login-btn", locales = ATTR_MACRO_LOCALES);
///
/// // Get an attribute with no arguments (using default LOCALES)
/// let idle_text = i18n::attr!(msg, "idle");
/// assert_eq!(idle_text, "Login");
///
/// // Get an attribute with arguments and a custom locales instance
/// let arg_text = i18n::attr!(msg, "attr-arg", locales = ATTR_MACRO_LOCALES, "text" = "some value");
/// assert_eq!(arg_text, "This is an attribute argument with arbitrary text: \u{2068}some value\u{2069}");
/// ```
#[proc_macro]
pub fn attr(input: TokenStream) -> TokenStream {
    attr::attr_impl(input)
}
