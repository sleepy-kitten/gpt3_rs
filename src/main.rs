#![feature(derive_default_enum)]
#![feature(crate_visibility_modifier)]

use crate::{
    api::{answers, classifications, completions},
    client::Client,
};

#[macro_use]
mod macros;
pub mod api;
pub mod client;
mod model;

const OPENAI_URL: &str = "https://api.openai.com/v1";
fn main() {
    println!("Hello, world!");
    //let client = Client::new(String::from("aaaa"));
    //answers::Request::builder().examples(examples)
    answers::Builder::default().documents(vec![String::new()]).build().unwrap();
    //client.request(completions::Request::builder().prompt(String::from("test")));
}
