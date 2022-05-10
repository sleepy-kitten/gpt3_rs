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

pub mod api;
mod client;
mod error;
mod into_vec;
mod model;
pub use client::Client;
pub use error::Error;
pub use model::Model;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::client::Client;
    pub use crate::error::Error;
    pub use crate::model::Model;
}

const OPENAI_URL: &str = "https://api.openai.com/v1";
