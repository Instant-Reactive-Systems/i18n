//! I18n utilties and tools.
pub use i18n_macros::langs;
pub use unic_langid::{langid, langids, LanguageIdentifier};

/// Localizes the type given a [`unic_langid::LanguageIdentifier`].
pub trait LocalizedDisplay {
    /// Localizes the type given a [`unic_langid::LanguageIdentifier`].
    fn localize(&self, lang: &LanguageIdentifier) -> String;
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

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_templates::Loader;

    mod i18n {
        pub use super::*;
    }

    #[test]
    fn test_if_type_works() {
        fluent_templates::static_loader! {
            pub static LOCALE = {
                locales: "i18n",
                fallback_language: "en-US",
            };
        }

        enum Foo {
            A,
            B,
            C,
        }

        impl LocalizedDisplay for Foo {
            fn localize(&self, lang: &LanguageIdentifier) -> String {
                match self {
                    Self::A => LOCALE.lookup(lang, "foo-a"),
                    Self::B => LOCALE.lookup(lang, "foo-b"),
                    Self::C => LOCALE.lookup(lang, "foo-c"),
                }
            }
        }

        let a = Foo::A;
        let b = Foo::B;

        let res = a.localize(&unic_langid::langid!("en-US"));
        assert_eq!(res, "English A");

        let res = a.localize(&unic_langid::langid!("hr-hr"));
        assert_eq!(res, "Croatian A");
    }

    #[test]
    fn test_if_langs_macro_works() {
        let langs = crate::langs!("./i18n");
        println!("{langs:?}");
    }
}
