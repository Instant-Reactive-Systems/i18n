//! I18n utilties and tools.

pub use i18n_lang::*;
pub use i18n_loader::*;
pub use i18n_macros::*;

/// Localizes the type given a `LanguageIdentifier`.
pub trait LocalizedDisplay {
    /// Localizes the type given a `LanguageIdentifier`.
    fn localize(&self, lang: &LanguageIdentifier) -> Message;
}
