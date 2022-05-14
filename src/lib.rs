//! # gpt3_rs
//!
//! This crate is for interacting with [OpenAi](https://openai.com)'s gpt-3.
//!
//! # Warning
//! There are a few apis missing and the documentation is incomplete
//!
//! # Examples
//! ```ignore
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
//!     let response = client.request(&request).await.unwrap();
//!     let answer = &response.choices[0].text;
//!
//!     println!("{answer}");
//! }
//! ```
#[cfg(not(feature = "blocking"))]
pub(crate) type RequestBuilder = reqwest::RequestBuilder;
#[cfg(feature = "blocking")]
pub(crate) type RequestBuilder = reqwest::blocking::RequestBuilder;

pub mod api;
mod client;
mod into_vec;
mod model;
pub use client::Client;
pub use model::Model;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::client::Client;
    pub use crate::model::Model;
}
#[cfg(test)]
#[cfg(feature = "blocking")]
mod tests {
    use crate::{client::Request, prelude::*};
    #[test]
    fn answers() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = answers::Builder::default()
            .model(Model::Ada)
            .search_model(Model::Ada)
            .question("which puppy is happy?")
            .documents(&["Puppy A is happy", "Puppy B is sad."])
            .examples_context("In 2017, U.S. life expectancy was 78.6 years.")
            .examples(&[&[
                "What is human life expectancy in the United States?",
                "78 years.",
            ]])
            .max_tokens(5)
            .stop(&["\n", "<|endoftext|>"])
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
    #[test]
    fn classifications() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = classifications::Builder::default()
            .model(Model::Ada)
            .search_model(Model::Ada)
            .query("It is a rainy day :(")
            .examples(&[
                &["A happy moment", "Positive"],
                &["I am sad.", "Negative"],
                &["I am feeling awesome", "Positive"],
            ])
            .labels(&["Positive", "Negative", "Neutral"])
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
    #[test]
    fn completions() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = completions::Builder::default()
            .model(Model::Ada)
            .prompt(&["Say this is a test"])
            .max_tokens(5)
            .temperature(1.0)
            .top_p(1.0)
            .n(1)
            .stop(&["\n"])
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
    #[test]
    fn edits() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = edits::Builder::default()
            .model(Model::Davinci)
            .input("What day of the wek is it?")
            .instruction("Fix the spelling mistakes")
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
    #[test]
    fn searches() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = searches::Builder::default()
            .model(Model::Ada)
            .documents(&["White house", "hospital", "school"])
            .query("the president")
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
    #[test]
    fn trait_impl() {
        let token = std::env::var("GPT_API_TOKEN").unwrap();

        let client = Client::new(token);

        let request = searches::Builder::default()
            .model(Model::Ada)
            .documents(&["White house", "hospital", "school"])
            .query("the president")
            .build()
            .unwrap();

        let response = request.request(&client).unwrap();
        println!("{:#?}", response);
    }
}
const OPENAI_URL: &str = "https://api.openai.com/v1";
