//! This module other modules that are used to interact with the api
//!
//! Each of the modules contains a `module::Request` struct, usually crated with the `module::Builder`
//! and a `module::Response` struct.
//!  

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::Client;
pub mod answers;
pub mod classifications;
pub mod completions;
pub mod edits;
pub mod files;
pub mod searches;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogProbs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<Option<f64>>,
    pub top_logprobs: Vec<Option<HashMap<String, f64>>>,
    pub text_offset: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Purpose {
    Search,
    Answers,
    Classifications,
    #[serde(rename = "fine-tune")]
    FineTuning,
}

#[doc(hidden)]
pub trait Action {
    type Response;
    fn build_request(&self, client: &Client) -> crate::RequestBuilder;
}
#[doc(hidden)]
pub trait RequestInfo {
    type Response;
    fn url(&self) -> String;
}
trait Auth {
    fn auth_header<T>(self, token: T) -> Self
    where
        T: std::fmt::Display;
    fn auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display;
}
impl Auth for crate::RequestBuilder {
    fn auth_header<T>(self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        self.header("Content-Type", "application/json")
            .bearer_auth(token)
    }
    fn auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        self.bearer_auth(token)
    }
}

impl<T> Action for T
where
    T: RequestInfo + Serialize,
{
    type Response = T::Response;
    fn build_request(&self, client: &Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .post(&self.url())
            .auth_header(client.gpt_token())
            .json(self)
    }
}
