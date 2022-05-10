# gpt3_rs

gpt3_rs is a rust library for interacting with OpenAi's gpt3

## features

- builder pattern to build request structs
- easy to use
- blazingly inefficient (:rocket:)

## Examples

```rust
use gpt3_rs::{Client, api::*, Model};

#[tokio::main]
async fn main() {
    let token = std::env::var("GPT_API_TOKEN").unwrap();
    let client = Client::new(token);

    let request = completions::Builder::default()
        .model(Model::Babbage)
        .prompt("what is 1 + 2?")
        .build()
        .unwrap();
    let response = client.request(&request).await.unwrap();
    let answer = &response.choices[0].text;

    println!("{answer}");
}
```

```rust
use gpt3_rs::{Client, api::*, Model};

#[tokio::main]
async fn main() {
    let token = std::env::var("GPT_API_TOKEN").unwrap();
    let client = Client::new(token);

    let request = classification::Builder::default()
        .model(Model::Curie)
        .search_model(Model::Ada)
        .query("It is a rainy day :(")
     .  .examples(&[
            &["A happy moment", "Positive"],
            &["I am sad.", "Negative"],
            &["I am feeling awesome", "Positive"]
        ])
        .labels(&["Positive", "Negative", "Neutral"])
        .build()
        .unwrap();

    let response = client.request(&request).await.unwrap();
    let answer = &response.choices[0].text;

    println!("{answer}");
}
l
```