# gpt3_rs

gpt3_rs is a rust library for interacting with OpenAi's gpt3

## features

- builder pattern to build request structs
- easy to use
- blazingly inefficient (:rocket:)

## Example

```rust
#[tokio::main]
async fn main() {
    let token = std::env::var("GPT_API_TOKEN").unwrap();
    let client = Client::new(token);

    let request = completions::Builder::default()
        .model(Model::Babbage)
        .prompt("what is 1 + 2?".into())
        .build()
        .unwrap();
    let response = client.request(request).await.unwrap();
    let answer = &response.choices[0].text;

    println!("{answer}");
}
```
