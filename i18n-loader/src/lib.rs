pub use fluent_bundle::{
    concurrent::FluentBundle,
    resolver::errors::{ReferenceKind, ResolverError},
    FluentArgs, FluentError, FluentResource, FluentValue,
};
pub use lazy_static;
use std::{borrow::Cow, collections::HashMap, sync::Arc};
pub use unic_langid::{langid, langids, LanguageIdentifier};

/// A thread-safe container for all loaded localization data.
///
/// It manages multiple `Locale` instances, keyed by language identifier,
/// and provides a unified interface for querying translations. It also handles
/// fallback logic to a default language if a translation is missing.
pub struct Locales {
    /// The map from a language identifier to its `Locale`.
    locales: HashMap<LanguageIdentifier, Locale>,
    /// The language to use as a fallback if a message is not found in the requested language.
    fallback_lang: LanguageIdentifier,
    /// An optional error handler to be called with any localization errors.
    on_error: Option<fn(&[FluentError])>,
}

impl Locales {
    /// Creates a new, empty `Locales` collection.
    ///
    /// # Arguments
    /// * `fallback_lang`: The language identifier to use if a translation is not found in the current language.
    /// * `on_error`: An optional callback function that will be invoked with any errors that occur during message formatting.
    pub fn new(fallback_lang: LanguageIdentifier, on_error: Option<fn(&[FluentError])>) -> Self {
        Self {
            locales: Default::default(),
            fallback_lang,
            on_error,
        }
    }

    /// Creates a new `Locales` collection from a network resource.
    ///
    /// # Arguments
    /// * `url`: The URL from which to fetch the translation.
    /// * `fallback_lang`: The language identifier to use if a translation is not found in the current language.
    /// * `on_error`: An optional callback function that will be invoked with any errors that occur during message formatting.
    #[cfg(feature = "net")]
    pub async fn from_url(
        url: &str,
        fallback_lang: LanguageIdentifier,
        on_error: Option<fn(&[FluentError])>,
    ) -> Result<Self, NetError> {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        let uri = url.parse().unwrap();

        let res = client
            .get(uri)
            .await
            .map_err(NetError::ServerError)?;
        let body = hyper::body::to_bytes(res.into_body())
            .await
            .map_err(NetError::ServerError)?;
        let definitions: HashMap<String, String> =
            serde_json::from_slice(&body).map_err(NetError::InvalidFormat)?;
        let mut parser_errors: Vec<ParserError> = Vec::default();
        let mut locales: HashMap<LanguageIdentifier, Locale> = HashMap::default();
        for (langid, definition) in definitions.into_iter() {
            let langid = match langid.parse::<LanguageIdentifier>() {
                Ok(langid) => langid,
                Err(_) => {
                    parser_errors.push(ParserError::InvalidLangid { langid });
                    continue;
                }
            };
            let resource = match FluentResource::try_new(definition) {
                Ok(resource) => resource,
                Err((_, errors)) => {
                    parser_errors.push(ParserError::ParserError { langid, errors });
                    continue;
                }
            };
            locales.insert(langid.clone(), Locale::new(langid, vec![resource]));
        }

        if !parser_errors.is_empty() {
            return Err(NetError::ParserError(parser_errors));
        }

        Ok(Self {
            locales,
            fallback_lang,
            on_error,
        })
    }

    /// Adds a new language's localization data to the collection.
    ///
    /// # Arguments
    /// * `lang_str`: A string slice representing the language identifier (e.g., "en-US", "de").
    /// * `resources`: A vector of `FluentResource`s containing the translation data for this language.
    ///
    /// # Panics
    /// Panics if `lang_str` is not a valid language identifier.
    pub fn add_locale(&mut self, lang_str: &str, resources: Vec<FluentResource>) {
        let lang_id: LanguageIdentifier = lang_str.parse().expect("Language ID should be valid");
        let locale = Locale::new(lang_id.clone(), resources);
        self.locales.insert(lang_id, locale);
    }

    /// Queries for a message in a specific language, applying fallback logic if the language is not found.
    ///
    /// It first attempts to find the `Locale` for the requested language. If the entire `Locale` is missing,
    /// it will automatically retry the query using the configured fallback language.
    #[track_caller]
    pub fn query(
        &self,
        lang: &LanguageIdentifier,
        query: &Query,
    ) -> Result<Message, Vec<FluentError>> {
        let query_result = match self.locales.get(lang) {
            Some(locale) => locale.query(query),
            None => {
                let fallback_locale = self
                    .locales
                    .get(&self.fallback_lang)
                    .expect("a fallback language should *always* exist and be present as a locale");
                fallback_locale.query(query)
            }
        };

        // inspect the errors if on_error exists
        if let (Some(on_error), Err(errs)) = (&self.on_error, &query_result) {
            on_error(errs);
        }
        query_result
    }

    /// If an `on_error` handler is configured, this method invokes it with the provided slice of `FluentError`s.
    pub fn call_on_error(&self, errors: &[FluentError]) {
        if let Some(on_error) = self.on_error {
            on_error(errors);
        }
    }
}

/// Manages Fluent localization resources for a specific locale.
///
/// A `Locale` holds a collection of `FluentResource` objects, which contain the
/// parsed data from `.ftl` (Fluent Translation List) files. It is responsible
/// for a single language and provides the resources needed to format localized
/// messages.
pub struct Locale {
    /// The underlying `FluentBundle` that manages the collection of resources
    /// and handles the formatting of messages.
    bundle: Arc<FluentBundle<Arc<FluentResource>>>,
}

impl Locale {
    /// Creates a new `Locale` for a given language and its resources.
    ///
    /// # Arguments
    /// * `lang`: The `LanguageIdentifier` for this locale.
    /// * `resources`: A vector of `FluentResource`s containing the translation data.
    pub fn new(lang: LanguageIdentifier, resources: Vec<FluentResource>) -> Self {
        let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
        for resource in resources.into_iter() {
            bundle
                .add_resource(Arc::new(resource))
                .expect("resource should never be overriding another; consider this a bug if it happens and open an issue at https://github.com/Instant-Reactive-Systems/i18n/issues");
        }
        let bundle = Arc::new(bundle);

        Self { bundle }
    }

    /// Resolves a `Query` into a fully formatted `Message`.
    ///
    /// This method takes a `Query` which specifies a message ID and any
    /// arguments, and attempts to format it into a `Message` struct.
    /// If the message ID is not found, or if any errors occur during formatting,
    /// an `Err` containing a vector of `FluentError`s is returned.
    #[track_caller]
    pub fn query(&self, query: &Query) -> Result<Message, Vec<FluentError>> {
        let mut errors = Vec::default();
        let msg = match self.bundle.get_message(&query.id) {
            Some(msg) => msg,
            None => {
                errors.push(FluentError::ResolverError(ResolverError::Reference(
                    ReferenceKind::Message {
                        id: query.id.to_string(),
                        attribute: None,
                    },
                )));
                return Err(errors);
            }
        };

        let value = match msg.value() {
            Some(pattern) => self
                .bundle
                .format_pattern(pattern, Some(&query.args), &mut errors)
                .to_string(),
            None => format!("<{}>", query.id),
        };

        let mut attrs = HashMap::default();
        for attr in msg.attributes() {
            let mut local_errors = Vec::default();
            let pattern = attr.value();
            let attr_cache = match query.attr_args.get(attr.id()) {
                Some(args) => {
                    let value = self
                        .bundle
                        .format_pattern(pattern, Some(args), &mut local_errors);

                    AttrCache {
                        entry_id: query.id.to_string(),
                        attr_id: attr.id().to_string(),
                        value: Some(value.to_string()),
                        bundle: self.bundle.clone(),
                    }
                }
                None => {
                    let mut even_more_local_errors = Vec::default();
                    let value =
                        self.bundle
                            .format_pattern(pattern, None, &mut even_more_local_errors);

                    let value = if !even_more_local_errors.is_empty() {
                        let only_missing_attr_args = even_more_local_errors.iter().all(|err| {
                            matches!(
                                err,
                                FluentError::ResolverError(ResolverError::Reference(
                                    ReferenceKind::Variable { .. }
                                ))
                            )
                        });

                        // only consider errors other than a missing placeable as an actual error
                        if !only_missing_attr_args {
                            local_errors.extend(even_more_local_errors.into_iter());
                        }

                        None
                    } else {
                        Some(value.to_string())
                    };

                    AttrCache {
                        entry_id: query.id.to_string(),
                        attr_id: attr.id().to_string(),
                        value,
                        bundle: self.bundle.clone(),
                    }
                }
            };

            attrs.insert(attr.id().to_string(), attr_cache);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Message {
            id: query.id.to_string(),
            value: value.to_string(),
            attrs,
        })
    }
}

/// Represents a localized message with its ID, value, and attributes.
#[derive(Debug, PartialEq, Default)]
pub struct Message {
    /// The unique identifier for the message (e.g., "login-button").
    pub id: String,
    /// The translated main text of the message.
    pub value: String,
    /// A map of associated attributes for the message, such as `aria-label`.
    pub attrs: HashMap<String, AttrCache>,
}

/// Represents a request to format a localized message, including its ID and arguments.
#[derive(Debug, Default)]
pub struct Query<'a> {
    /// The ID of the message to format (e.g., "hello-world").
    id: Cow<'a, str>,
    /// Arguments for the main message value.
    args: FluentArgs<'a>,
    /// Arguments for specific message attributes, keyed by attribute name.
    attr_args: HashMap<Cow<'a, str>, FluentArgs<'a>>,
    /// A flag to indicate whether to use the fallback language if the message is not found.
    with_fallback: bool,
}

impl<'a> Query<'a> {
    /// Creates a new `Query` for a given message ID.
    pub fn new(id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            id: id.into(),
            args: Default::default(),
            attr_args: Default::default(),
            with_fallback: false,
        }
    }

    /// Adds an argument for the main message value.
    ///
    /// # Example
    ///
    /// ```
    /// use i18n_loader::Query;
    ///
    /// let query = Query::new("hello-user").with_arg("userName", "Alex");
    /// ```
    pub fn with_arg<I, V>(mut self, id: I, value: V) -> Self
    where
        I: Into<Cow<'a, str>>,
        V: Into<FluentValue<'a>>,
    {
        self.args.set(id.into(), value.into());
        self
    }

    /// Adds an argument for a specific attribute of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use i18n_loader::Query;
    ///
    /// let query = Query::new("user-tooltip").with_attr_arg("aria-label", "userName", "Alex");
    /// ```
    pub fn with_attr_arg<A, I, V>(mut self, attr: A, id: I, value: V) -> Self
    where
        A: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>,
        V: Into<FluentValue<'a>>,
    {
        self.attr_args
            .entry(attr.into())
            .or_default()
            .set(id.into(), value.into());
        self
    }

    /// Enables or disables fallback to the default language for this specific query.
    ///
    /// If set to `true`, and the requested message is not found in the primary language,
    /// the query will be re-attempted using the `Locales` fallback language.
    pub fn with_fallback(mut self, enable_fallback: bool) -> Self {
        self.with_fallback = enable_fallback;
        self
    }
}

/// Provides a cache for the value of a specific attribute from a localization entry.
///
/// This struct is designed for lazy evaluation. It stores the identifiers for a
/// message's attribute (`entry_id`, `attr_id`) and a static reference to the `Locales`
/// instance. The actual localized string is only fetched from the origin `Locales`
/// and stored in `value` upon first access, reducing overhead for attributes that
/// are not immediately needed.
pub struct AttrCache {
    /// The ID of the main localization entry.
    pub entry_id: String,
    /// The ID of the attribute entry.
    pub attr_id: String,
    /// The cached value of the localization.
    pub value: Option<String>,
    /// The underlying `FluentBundle` that manages the collection of resources
    /// and handles the formatting of messages.
    pub bundle: Arc<FluentBundle<Arc<FluentResource>>>,
}

impl AttrCache {
    /// Queries the cached attribute, formatting it with the given arguments.
    ///
    /// If the attribute value is already cached, it will be returned immediately.
    /// Otherwise, it will be formatted using the provided `args`. If `args` are
    /// not provided, any arguments required by the attribute will be missing,
    /// potentially resulting in a formatting error.
    ///
    /// # Errors
    /// Returns a `Vec<FluentError>` if any errors occur during formatting, such as
    /// missing message IDs, attributes, or arguments.
    pub fn query(&mut self, args: Option<&FluentArgs>) -> Result<String, Vec<FluentError>> {
        // return the cached localization
        if let Some(value) = self.value.clone() {
            return Ok(value);
        }

        let mut errors = Vec::default();
        let msg = match self.bundle.get_message(&self.entry_id) {
            Some(msg) => msg,
            None => {
                errors.push(FluentError::ResolverError(ResolverError::Reference(
                    ReferenceKind::Message {
                        id: self.entry_id.to_string(),
                        attribute: None,
                    },
                )));
                return Err(errors);
            }
        };

        let Some(this_attr) = msg.attributes().find(|attr| attr.id() == self.attr_id) else {
            errors.push(FluentError::ResolverError(ResolverError::Reference(
                ReferenceKind::Message {
                    id: self.entry_id.to_string(),
                    attribute: Some(self.attr_id.to_string()),
                },
            )));
            return Err(errors);
        };

        let pattern = this_attr.value();
        let value = self.bundle.format_pattern(pattern, args, &mut errors);

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(value.to_string())
    }
}

/// Implements equality for `AttrCache`.
///
/// Two `AttrCache` instances are considered equal if they point to the same
/// message entry and attribute. The cached `value` is not considered in the
/// comparison.
impl PartialEq for AttrCache {
    fn eq(&self, other: &Self) -> bool {
        self.entry_id == other.entry_id && self.attr_id == other.attr_id
    }
}

/// Custom debug implementation for `AttrCache`.
///
/// This implementation provides a clean debug output, showing the entry ID,
/// attribute ID, and the cached value if present.
impl std::fmt::Debug for AttrCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttrCache")
            .field("entry_id", &self.entry_id)
            .field("attr_id", &self.attr_id)
            .field("value", &self.value)
            .finish()
    }
}

#[cfg(feature = "net")]
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("could not parse langid: {langid}")]
    InvalidLangid { langid: String },
    #[error("errors occurred during parsing of {}", {
        use itertools::Itertools;
        format!(
            "{langid}:\n{}",
            errors.iter().map(|err| format!("\t- {err:?}")).join("\n")
        )
    })]
    ParserError {
        langid: LanguageIdentifier,
        errors: Vec<fluent_syntax::parser::ParserError>,
    },
}

#[cfg(feature = "net")]
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error(transparent)]
    ServerError(#[from] hyper::Error),
    #[error("errors occurred during parsing:\n{}", {
        use itertools::Itertools;
        _0.iter().join("\n")
    })]
    ParserError(Vec<ParserError>),
    #[error("invalid format received from server, expected {{'lang-id': 'fluent-definitions', ..}}; errors: {0}")]
    InvalidFormat(#[from] serde_json::Error),
}
