#[test]
fn test_if_type_works() {
    use i18n::LocalizedDisplay;
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    enum Foo {
        A,
        B,
        C,
    }

    impl i18n::LocalizedDisplay for Foo {
        fn localize(&self, lang: &i18n::LanguageIdentifier) -> i18n::Message {
            match self {
                Self::A => LOCALES.query(lang, &i18n::Query::new("foo-a")).unwrap(),
                Self::B => LOCALES.query(lang, &i18n::Query::new("foo-b")).unwrap(),
                Self::C => LOCALES.query(lang, &i18n::Query::new("foo-c")).unwrap(),
            }
        }
    }

    let a = Foo::A;

    let res = a.localize(&i18n::langid!("en-US"));
    assert_eq!(res.value, "English A");

    let res = a.localize(&i18n::langid!("hr-hr"));
    assert_eq!(res.value, "Croatian A");
}

#[test]
fn test_if_langs_macro_works() {
    let langs = i18n::langs!("./tests/i18n");
    println!("{langs:?}");
}

#[test]
fn test_if_failures_are_reported() {
    // i18n::load!("./tests/i18n_fail", fallback_lang = "en-US");
    // i18n::load!("./tests/i18n_fail", fallback_lang = "en-US", check_keys = true);
}

#[test]
fn test_if_arguments_work() {
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let query = i18n::Query::new("welcome-back").with_arg("username", "John");
    let msg = LOCALES.query(&lang, &query).unwrap();
    assert_eq!(
        msg,
        i18n::Message {
            id: "welcome-back".to_string(),
            value: "Welcome back, \u{2068}John\u{2069}!".to_string(),
            attrs: Default::default(),
        }
    )
}

#[test]
fn test_if_attributes_work() {
    use std::collections::HashMap;
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let query =
        i18n::Query::new("login-btn").with_attr_arg("attr-arg", "text", "this is arbitrary text");
    let msg = LOCALES.query(&lang, &query).unwrap();
    assert_eq!(
        msg,
        i18n::Message {
            id: "login-btn".to_string(),
            value: "".to_string(),
            attrs: HashMap::from_iter([
                ("idle", "Login"),
                ("progress", "Logging in..."),
                ("finished-ok", "Logged in"),
                ("finished-err", "Failed"),
                ("aria-label", "A login button"),
                ("attr-arg", "This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}"),
            ].map(|(attr, value)| (attr.to_string(), value.to_string()))),
        }
    )
}

#[test]
fn test_if_tr_macro_works() {
    use std::collections::HashMap;
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let msg = i18n::tr!(
        lang,
        "login-btn",
        attr("attr-arg", text = "this is arbitrary text")
    );
    assert_eq!(
        msg,
        i18n::Message {
            id: "login-btn".to_string(),
            value: "".to_string(),
            attrs: HashMap::from_iter([
                ("idle", "Login"),
                ("progress", "Logging in..."),
                ("finished-ok", "Logged in"),
                ("finished-err", "Failed"),
                ("aria-label", "A login button"),
                ("attr-arg", "This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}"),
            ].map(|(attr, value)| (attr.to_string(), value.to_string()))),
        }
    )
}

#[test]
fn test_tr_macro_with_args_no_locales_on_error() {
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let msg = i18n::tr!(lang, "welcome-back", username = "Alice");
    assert_eq!(
        msg,
        i18n::Message {
            id: "welcome-back".to_string(),
            value: "Welcome back, \u{2068}Alice\u{2069}!".to_string(),
            attrs: Default::default(),
        }
    );
}