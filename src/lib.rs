mod client;
mod model;
pub mod api;
pub use client::Client;
pub use model::Model;

const OPENAI_URL: &str = "https://api.openai.com/v1";
