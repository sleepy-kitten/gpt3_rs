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
impl ToString for Purpose {
    fn to_string(&self) -> String {
        match self {
            Purpose::Search => "search",
            Purpose::Answers => "answers",
            Purpose::Classifications => "classifications",
            Purpose::FineTuning => "fine-tune",
        }
        .to_string()
    }
}

#[doc(hidden)]
pub trait BuildRequest {
    fn build_request(&self, client: &Client) -> crate::RequestBuilder;
}
#[doc(hidden)]
pub trait RequestInfo {
    fn url(&self) -> String;
}

impl<T> BuildRequest for T
where
    T: RequestInfo + Serialize,
{
    fn build_request(&self, client: &Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .post(&self.url())
            .bearer_auth(client.gpt_token())
            .json(self)
    }
}
