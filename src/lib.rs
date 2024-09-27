//! I18n utilties and tools.

/// Localizes the type given a [`fluent_templates::Loader`].
///
/// [`fluent_templates::Loader`]: https://docs.rs/fluent-templates/latest/fluent_templates/trait.Loader.html
pub trait LocalizedDisplay {
    fn localize(&self, locale: &dyn fluent_templates::Loader) -> String;
}
