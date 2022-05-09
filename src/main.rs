#![feature(derive_default_enum)]
#![feature(crate_visibility_modifier)]
#[macro_use]
pub mod traits;

pub mod action;
mod model;
pub mod client;

const OPENAI_URL: &str = "https://api.openai.com/v1";
fn main() {
    println!("Hello, world!");
}

