//! I18n utilties and tools.

pub use i18n_loader::*;
pub use i18n_macros::*;

/// Localizes the type given a `LanguageIdentifier`.
pub trait LocalizedDisplay {
    /// Localizes the type given a `LanguageIdentifier`.
    fn localize(&self, lang: &LanguageIdentifier) -> Message;
}

/// Provides all information on a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lang {
    pub id: &'static str,
    pub name: &'static str,
    pub flag: &'static str,
    pub dir: &'static str,
}

impl std::hash::Hash for Lang {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
