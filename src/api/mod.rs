use reqwest::RequestBuilder;
use serde::Serialize;

use crate::client::Client;

pub mod answers;
pub mod classifications;
pub mod completions;
pub mod edits;
pub mod files;
pub mod searches;

pub trait Action {
    type Response;
    fn build_request(&self, client: &Client) -> RequestBuilder;
}
pub trait RequestInfo {
    type Response;
    fn url(&self) -> String;
}

impl<T> Action for T
where
    T: RequestInfo + Serialize,
{
    type Response = T::Response;
    fn build_request(&self, client: &Client) -> reqwest::RequestBuilder {
        client
            .init_post(&self.url())
            .body(serde_json::to_string(self).unwrap())
    }
}
