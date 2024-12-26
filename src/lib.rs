//! I18n utilties and tools.

/// Localizes the type given a [`fluent_templates::Loader`].
///
/// [`fluent_templates::Loader`]: https://docs.rs/fluent-templates/latest/fluent_templates/trait.Loader.html
pub trait LocalizedDisplay {
    fn localize(&self, lang: &fluent_templates::LanguageIdentifier) -> String;
}

// #[cfg(tests)]
mod tests {
    use super::*;
    use fluent_templates::Loader;

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
            fn localize(&self, lang: &fluent_templates::LanguageIdentifier) -> String {
                match self {
                    Self::A => LOCALE.lookup(lang, "foo-a"),
                    Self::B => LOCALE.lookup(lang, "foo-b"),
                    Self::C => LOCALE.lookup(lang, "foo-c"),
                }
            }
        }

        let a = Foo::A;
        let b = Foo::B;

        let res = a.localize(&fluent_templates::langid!("en-US"));
        assert_eq!(res, "English A");

        let res = a.localize(&fluent_templates::langid!("hr-hr"));
        assert_eq!(res, "Croatian A");
    }
}
