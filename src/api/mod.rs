use reqwest::RequestBuilder;
use serde::Serialize;

use crate::client::Client;

mod answers;
mod classifications;
mod completions;
mod edits;
mod searches;

pub trait Action {
    fn build_request(&self, client: &Client) -> RequestBuilder;
}
trait Url {
    fn url(&self) -> String;
}
impl<T> Action for T
where
    T: Url + Serialize,
{
    fn build_request(&self, client: &Client) -> reqwest::RequestBuilder {
        client
            .init_post(&self.url())
            .body(serde_json::to_string(self).unwrap())
    }
}
