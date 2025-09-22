pub use fluent_bundle::{
    concurrent::FluentBundle,
    resolver::errors::{ReferenceKind, ResolverError},
    FluentArgs, FluentError, FluentResource, FluentValue,
};
pub use lazy_static;
pub use unic_langid::{langid, langids, LanguageIdentifier};

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// A collection of `Locale` instances, keyed by language.
pub struct Locales {
    /// The map from a language identifier to its `Locale`.
    locales: HashMap<LanguageIdentifier, Locale>,
    /// The language to use as a fallback if a message is not found in the requested language.
    fallback_lang: LanguageIdentifier,
    /// An optional error handler to be called with any localization errors.
    on_error: Option<fn(&[FluentError])>,
}

impl Locales {
    /// Creates a new, empty `Locales` collection with a specified fallback language.
    pub fn new(fallback_lang: LanguageIdentifier, on_error: Option<fn(&[FluentError])>) -> Self {
        Self {
            locales: Default::default(),
            fallback_lang,
            on_error,
        }
    }

    /// Adds a new locale to the collection from a set of resources.
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
            on_error(&errs);
        }
        return query_result;
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
    bundle: FluentBundle<Arc<FluentResource>>,
}

impl Locale {
    /// Creates a new `Locale` for a given language and resource.
    pub fn new(lang: LanguageIdentifier, resources: Vec<FluentResource>) -> Self {
        let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
        for resource in resources.into_iter() {
            bundle
                .add_resource(Arc::new(resource))
                .expect("resource should never be overriding another; consider this a bug if it happens and open an issue at https://github.com/Instant-Reactive-Systems/i18n/issues");
        }
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
            None => Default::default(),
        };

        let mut query_attrs = query.attr_args.keys().cloned().collect::<HashSet<_>>();
        let mut attrs = HashMap::default();
        for attr in msg.attributes() {
            let pattern = attr.value();
            let attr_args = query.attr_args.get(attr.id());
            if attr_args.is_some() {
                query_attrs.remove(attr.id());
            }
            let mut local_errors = Vec::default();
            let value = self
                .bundle
                .format_pattern(pattern, attr_args, &mut local_errors);

            // we ignore variable errors if no variables were provided by us
            if attr_args.is_none() {
                for err in local_errors.into_iter() {
                    if let FluentError::ResolverError(ResolverError::Reference(
                        ReferenceKind::Variable { .. },
                    )) = err
                    {
                        continue;
                    }

                    errors.push(err);
                }
            } else {
                errors.extend(local_errors.into_iter());
            }
            attrs.insert(attr.id().to_string(), value.to_string());
        }

        for attr in query_attrs.into_iter() {
            errors.push(FluentError::ResolverError(ResolverError::Reference(
                ReferenceKind::Message {
                    id: query.id.to_string(),
                    attribute: Some(attr.to_string()),
                },
            )));
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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Message {
    /// The unique identifier for the message (e.g., "login-button").
    pub id: String,
    /// The translated main text of the message.
    pub value: String,
    /// A map of associated attributes for the message, such as `aria-label`.
    pub attrs: HashMap<String, String>,
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

    /// Enables or disables fallback behavior for this query.
    pub fn with_fallback(mut self, enable_fallback: bool) -> Self {
        self.with_fallback = enable_fallback;
        self
    }
}
