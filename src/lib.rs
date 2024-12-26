//! I18n utilties and tools.

/// Localizes the type given a [`fluent_templates::Loader`].
///
/// [`fluent_templates::Loader`]: https://docs.rs/fluent-templates/latest/fluent_templates/trait.Loader.html
pub trait LocalizedDisplay {
    fn localize(
        &self,
        lang: &fluent_templates::LanguageIdentifier,
        locale: &dyn fluent_templates::Loader,
    ) -> String;
}

// #[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_if_type_works() {
        enum Foo {
            A,
            B,
            C,
        }

        impl LocalizedDisplay for Foo {
            fn localize(
                &self,
                lang: &fluent_templates::LanguageIdentifier,
                locale: &dyn fluent_templates::Loader,
            ) -> String {
                match self {
                    Self::A => locale.lookup(lang, "foo-a"),
                    Self::B => locale.lookup(lang, "foo-b"),
                    Self::C => locale.lookup(lang, "foo-c"),
                }
            }
        }

        fluent_templates::static_loader! {
            pub static LOCALE = {
                locales: "i18n",
                fallback_language: "en-US",
            };
        }

        let a = Foo::A;
        let b = Foo::B;

        let res = a.localize(&fluent_templates::langid!("en-US"), &*LOCALE);
        assert_eq!(res, "English A");

        let res = a.localize(&fluent_templates::langid!("hr-hr"), &*LOCALE);
        assert_eq!(res, "Croatian A");
    }
}
