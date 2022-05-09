use reqwest::RequestBuilder;

use crate::client::Client;

mod answer;
mod classification;
mod completion;
mod edit;
mod search;

pub trait Action {
    const CONTENT_TYPE: &'static str = "Content-Type: application/json";
    fn build_request(&self, client: &Client) -> RequestBuilder;
}
