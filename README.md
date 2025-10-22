# i18n - A Modern Internationalization Library for Rust

<!-- [![Crates.io](https://img.shields.io/crates/v/i18n.svg)](https://crates.io/crates/i18n) -->
[![Docs](https://img.shields.io/badge/docs-passing-passing?color=blue)](https://instant-reactive-systems.github.io/i18n/i18n/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-APACHE)

A modern, ergonomic, and powerful internationalization (i18n) library for Rust, built on top of [Mozilla's Fluent](https://projectfluent.org/) project. This library provides a complete solution for translating applications, from compile-time checked messages to a flexible runtime loader.

---

## Key Features

*   **Powerful Fluent Syntax**: Leverages Mozilla's Fluent specification, which allows for expressive translations with support for pluralization, gender, and complex interpolations.
*   **Compile-Time Safety**: The `i18n-macros` crate ensures that all your translation keys are valid at compile time, eliminating a whole class of runtime errors.
*   **Flexible Runtime Loader**: The `i18n-loader` crate provides a `Locales` container that can be used to dynamically load and manage translation resources.
*   **Network Loading**: (Optional `net` feature) Load your localization resources directly from a URL at runtime.
*   **Ergonomic API**: A simple and intuitive API makes it easy to integrate i18n into any application.

## Crates

This repository is a Cargo workspace containing the following crates:

| Crate         | Description                                                                                             |
|---------------|---------------------------------------------------------------------------------------------------------|
| `i18n-loader` | The runtime core of the library. It provides the `Locales` struct for loading and managing Fluent resources. |
| `i18n-macros` | Provides the procedural macros (e.g., `tr!`) for compile-time checked translations and resource embedding. |
| `i18n`        | The umbrella crate that conveniently re-exports the most common items from the other crates.             |

## Getting Started

### 1. Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
i18n = { git = "https://github.com/Instant-Reactive-Systems/i18n", tag = "0.1" }
```

### 2. Create Localization Files

Create a directory to store your Fluent (`.ftl`) files.

**`i18n/en-US/main.ftl`**
```ftl
hello-world = Hello, world!
hello-user = Hello, { $userName }!
```

**`i18n/hr-HR/main.ftl`**
```ftl
hello-world = Bok, svijete!
hello-user = Bok, { $userName }!
```

### 3. Load and Use Translations

Use the `load!` macro to access your translations. The macro will automatically handle loading the `Locales` container.
Then, use the `tr!` macro to query the messages.

```rust
fn main() {
    // Automatically parses all the files and maps
    // them to their language identifiers accordingly.
    i18n::load!("./i18n");

    // langs
    let en = i18n::langid!("en-US");
    let hr = i18n::langid!("hr-hr");

    // Simple translation
    let greeting_en = i18n::tr!(en, "hello-world");
    let greeting_hr = i18n::tr!(hr, "hello-world");

    println!("{}", greeting_en.value); // -> "Hello, world!"
    println!("{}", greeting_hr.value); // -> "Bok, svijete!"

    // Translation with arguments
    let user_greeting = i18n::tr!(en, "hello-user", "userName" = "Alex");
    println!("{}", user_greeting.value); // -> "Hello, Alex!"

    // Translation with attributes
    let confirmation_modal = i18n::tr!(en, "confirmation-modal");
    println!("{}", confirmation_modal.value); // -> "Are you sure you want to leave?"
    println!("{}", confirmation_modal.attrs["confirm"].value); // -> "Confirm"
    println!("{}", i18n::attr!(confirmation_modal, "signed_out_from", "email" = "test@mail.com")); // -> "You will be signed out of all accounts logged in with test@mail.com."
}
```

## Advanced Usage

### Loading from a URL (`net` feature)

Enable the `net` feature in your `Cargo.toml`:

```toml
[dependencies]
i18n = { git = "https://github.com/Instant-Reactive-Systems/i18n", tag = "0.1", features = ["net"] }
```

You can then use `Locales::from_url` to initialize your translations from a remote JSON file that maps language IDs to Fluent resource strings.

```rust
#[tokio::main]
async fn main() {
    let fallback_lang = langid!("en-US");
    let locales = Locales::from_url("https://example.com/locales.json", fallback_lang, None)
        .await
        .expect("Failed to load locales");

    // ... use locales as normal
}
```

The `locales.json` file should be in the following format:
```json
{
  "en-US": "hello-world = Hello from the web!",
  "de": "hello-world = Hallo aus dem Web!",
  "<language>": "<fluent-definitions>"
}
```

## Contributing

Contributions are welcome! If you have a feature request, bug report, or pull request, please feel free to open an issue or PR.

### Running Tests

To run the test suite for the entire workspace, execute the following command from the root of the repository:

```sh
cargo test --workspace
```

## License

This project is licensed under the MIT license.
