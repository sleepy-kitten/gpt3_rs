//! # gpt3_rs
//!
//! This crate is for interacting with [OpenAi](https://openai.com)'s gpt-3.
//!
//! # Warning
//! There are a few apis missing and the documentation is incomplete
//!
//! # Examples
//! ```
//! #[tokio::main]
//! async fn main() {
//!     let token = std::env::var("GPT_API_TOKEN").unwrap();
//!
//!     let client = Client::new(token);
//!
//!     let request = completions::Builder::default()
//!         .model(Model::Babbage)
//!         .prompt("what is 1 + 2?".into())
//!         .build()
//!         .unwrap();
//!
//!     let response = client.request(request).await.unwrap();
//!     let answer = &response.choices[0].text;
//!
//!     println!("{answer}");
//! }
//! ```

mod client;
mod model;
mod into_vec;
pub mod api;
pub use client::Client;
pub use model::Model;

const OPENAI_URL: &str = "https://api.openai.com/v1";
