use std::collections::HashMap;

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
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let query =
        i18n::Query::new("login-btn").with_attr_arg("attr-arg", "text", "this is arbitrary text");
    let msg = LOCALES.query(&lang, &query).unwrap();
    // get the bundle
    let bundle = msg
        .attrs
        .get("progress")
        .expect("should exist")
        .bundle
        .clone();
    assert_eq!(
        msg,
        i18n::Message {
            id: "login-btn".to_string(),
            value: "<login-btn>".to_string(),
            attrs: HashMap::from_iter([
                ("idle", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "idle".into(), value: Some("Login".into()), bundle: bundle.clone() }),
                ("progress", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "progress".into(), value: Some("Logging in...".into()), bundle: bundle.clone() }),
                ("finished-ok", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "finished-ok".into(), value: Some("Logged in".into()), bundle: bundle.clone() }),
                ("finished-err", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "finished-err".into(), value: Some("Failed".into()), bundle: bundle.clone() }),
                ("aria-label", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "aria-label".into(), value: Some("A login button".into()), bundle: bundle.clone() }),
                ("attr-arg", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "attr-arg".into(), value: Some("This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}".into()), bundle: bundle.clone() }),
            ].map(|(attr, value)| (attr.to_string(), value))),
        }
    )
}

#[test]
fn test_if_tr_macro_works() {
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let msg = i18n::tr!(
        lang,
        "login-btn",
        attr("attr-arg", "text" = "this is arbitrary text")
    );
    // get the bundle
    let bundle = msg
        .attrs
        .get("progress")
        .expect("should exist")
        .bundle
        .clone();
    assert_eq!(
        msg,
        i18n::Message {
            id: "login-btn".to_string(),
            value: "<login-btn>".to_string(),
            attrs: HashMap::from_iter([
                ("idle", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "idle".into(), value: Some("Login".into()), bundle: bundle.clone() }),
                ("progress", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "progress".into(), value: Some("Logging in...".into()), bundle: bundle.clone() }),
                ("finished-ok", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "finished-ok".into(), value: Some("Logged in".into()), bundle: bundle.clone() }),
                ("finished-err", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "finished-err".into(), value: Some("Failed".into()), bundle: bundle.clone() }),
                ("aria-label", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "aria-label".into(), value: Some("A login button".into()), bundle: bundle.clone() }),
                ("attr-arg", i18n::AttrCache { entry_id: "login-btn".into(), attr_id: "attr-arg".into(), value: Some("This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}".into()), bundle: bundle.clone() }),
            ].map(|(attr, value)| (attr.to_string(), value))),
        }
    )
}

#[test]
fn test_if_lazily_queried_attr_works() {
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let query = i18n::Query::new("login-btn");
    let mut msg = LOCALES.query(&lang, &query).unwrap();
    // get the bundle
    let bundle = msg
        .attrs
        .get("progress")
        .expect("should exist")
        .bundle
        .clone();
    assert_eq!(
        msg,
        i18n::Message {
            id: "login-btn".to_string(),
            value: "<login-btn>".to_string(),
            attrs: HashMap::from_iter(
                [
                    (
                        "idle",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "idle".into(),
                            value: Some("Login".into()),
                            bundle: bundle.clone()
                        }
                    ),
                    (
                        "progress",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "progress".into(),
                            value: Some("Logging in...".into()),
                            bundle: bundle.clone()
                        }
                    ),
                    (
                        "finished-ok",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "finished-ok".into(),
                            value: Some("Logged in".into()),
                            bundle: bundle.clone()
                        }
                    ),
                    (
                        "finished-err",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "finished-err".into(),
                            value: Some("Failed".into()),
                            bundle: bundle.clone()
                        }
                    ),
                    (
                        "aria-label",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "aria-label".into(),
                            value: Some("A login button".into()),
                            bundle: bundle.clone()
                        }
                    ),
                    (
                        "attr-arg",
                        i18n::AttrCache {
                            entry_id: "login-btn".into(),
                            attr_id: "attr-arg".into(),
                            value: None,
                            bundle: bundle.clone()
                        }
                    ),
                ]
                .map(|(attr, value)| (attr.to_string(), value))
            ),
        }
    );

    // test if passing no arg when there is one fails
    let attr = msg.attrs.get_mut("attr-arg").expect("should exist");
    assert_eq!(
        attr.query(None),
        Err(vec![i18n::FluentError::ResolverError(
            i18n::ResolverError::Reference(i18n::ReferenceKind::Variable { id: "text".into() })
        )]),
    );

    let attr = msg.attrs.get_mut("attr-arg").expect("should exist");
    let mut args = i18n::FluentArgs::default();
    args.set("text", "this is arbitrary text");
    let attr = attr.query(Some(&args));
    assert_eq!(attr, Ok("This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}".into()));
}

#[test]
fn test_if_attr_macro_works() {
    i18n::load!("./tests/i18n", fallback_lang = "en-US");

    let lang = i18n::langid!("en-US");
    let mut msg = i18n::tr!(lang, "login-btn");
    let attr = i18n::attr!(msg, "attr-arg", "text" = "this is arbitrary text");
    assert_eq!(
        attr,
        "This is an attribute argument with arbitrary text: \u{2068}this is arbitrary text\u{2069}"
    );
}
