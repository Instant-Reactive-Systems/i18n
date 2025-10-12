//! Provides mapping of language identifiers to information pertaining to the country.

mod langid_to_country_flag;
mod langid_to_dir;
mod langid_to_name;
pub use langid_to_country_flag::*;
pub use langid_to_dir::*;
pub use langid_to_name::*;
use unic_langid::LanguageIdentifier;

/// Provides all information on a language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lang {
    pub id: String,
    pub name: &'static str,
    pub flag: &'static str,
    pub dir: &'static str,
}

impl Lang {
    /// A new `Lang` object.
    pub fn new(langid: LanguageIdentifier) -> Self {
        Self::from(langid)
    }
}

impl From<LanguageIdentifier> for Lang {
    fn from(value: LanguageIdentifier) -> Self {
        let langid = value.to_string();
        let splitter = if langid.contains('_') {
            "_".to_string()
        } else if langid.contains('-') {
            "-".to_string()
        } else {
            langid.clone()
        };
        let mut parts = langid.split(&splitter);
        let langid = parts
            .next()
            .map(str::to_lowercase)
            .expect("should always be present");
        let region = parts.next().map(str::to_uppercase);
        let full_langid = if let Some(region) = &region {
            format!("{}-{}", langid, region)
        } else {
            langid.clone()
        };
        let name = langid_to_name(&langid);
        let flag = region
            .map(|region| langid_to_flag(&region))
            .flatten()
            .unwrap_or_default();
        let dir = langid_to_dir(&langid);

        Self {
            id: full_langid,
            name,
            flag,
            dir,
        }
    }
}

impl std::hash::Hash for Lang {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_parsing_from_a_langid_works() {
        let lang = crate::Lang::new(unic_langid::langid!("en-US"));
        assert_eq!(
            lang,
            crate::Lang {
                id: "en-US".to_string(),
                name: "English",
                flag: "🇺🇸",
                dir: "ltr"
            },
        );
    }
}
