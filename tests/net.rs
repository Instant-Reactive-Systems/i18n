use i18n_loader::{langid, Locales, Query};
use serde_json::json;

#[tokio::test]
async fn test_from_url_success() {
    let mut server = mockito::Server::new_async().await;

    // The content is a JSON object mapping langids to FTL strings.
    let json_content = json!({
        "en-US": "hello-world = Hello, world!",
        "hr-HR": "hello-world = Bok, svijete!"
    });

    let mock = server
        .mock("GET", "/locales.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json_content.to_string())
        .create_async()
        .await;

    let url = &format!("{}/locales.json", server.url());
    let fallback_lang = langid!("en-US");

    let locales_result = Locales::from_url(url, fallback_lang, None).await;

    // Assert that the request was successful and the Locales object was created.
    assert!(locales_result.is_ok());
    mock.assert_async().await;

    // Check if the locales were loaded correctly by querying for a message.
    let locales = locales_result.unwrap();

    let msg = locales
        .query(&langid!("en-US"), &Query::new("hello-world"))
        .unwrap();
    assert_eq!(msg.value, "Hello, world!");

    let msg = locales
        .query(&langid!("hr-HR"), &Query::new("hello-world"))
        .unwrap();
    assert_eq!(msg.value, "Bok, svijete!");
}
