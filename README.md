# zenn-rs

Unofficial ZennAPI Client.

## Features

- Fetch articles by user_name.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zenn = "0.1.0"
```

## Usage

```rust
use zenn::articles::fetch_articles;
use zenn::types::ArticleSearchParams;

#[tokio::main]
async fn main() {
    let search_params = ArticleSearchParams {
        username: "example_user".to_string(),
        count: 10,
        order: "latest".to_string(),
    };

    let result = fetch_articles(search_params).await;

    match result {
        Ok(data) => println!("Fetched articles: {:?}", data),
        Err(err) => eprintln!("Error fetching articles: {:?}", err),
    }
}
```

## Contributing

Contributions are welcome! If you find a bug or have an idea for a new feature, please [open an issue](https://github.com/ryohidaka/zenn-rs/issues) or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
