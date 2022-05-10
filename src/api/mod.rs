//! This module other modules that are used to interact with the api
//!
//! Each of the modules contains a `module::Request` struct, usually crated with the `module::Builder`
//! and a `module::Response` struct.
//!  

use reqwest::RequestBuilder;
use serde::Serialize;

use crate::client::Client;
pub mod answers;
pub mod classifications;
pub mod completions;
pub mod edits;
pub mod files;
pub mod searches;

#[doc(hidden)]
pub trait Action {
    type Response;
    fn build_request(&self, client: &Client) -> RequestBuilder;
}
#[doc(hidden)]
pub trait RequestInfo {
    type Response;
    fn url(&self) -> String;
}
trait Auth {
    fn auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display;
}
impl Auth for RequestBuilder {
    fn auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        self.header("Content-Type", "application/json")
            .bearer_auth(token)
    }
}

impl<T> Action for T
where
    T: RequestInfo + Serialize,
{
    type Response = T::Response;
    fn build_request(&self, client: &Client) -> reqwest::RequestBuilder {
        client
            .reqwest_client()
            .post(&self.url())
            .auth(client.gpt_token())
            .json(self)
    }
}
