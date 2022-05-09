use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::Client;
use crate::model::Model;

use super::Action;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Request {
    #[serde(skip)]
    pub model: Model,
    /// Query to search against the documents.
    pub query: String,
    /// Up to 200 documents to search over, provided as a list of strings.
    ///The maximum document length (in tokens) is 2034 minus the number of tokens in the query.
    ///You should specify either documents or a file, but not both.
    pub documents: Option<Vec<String>>,
    /// The ID of an uploaded file that contains documents to search over.
    /// You should specify either documents or a file, but not both.
    pub file: Option<String>,
    /// The maximum number of documents to be re-ranked and returned by search.
    /// This flag only takes effect when file is set.
    pub max_rerank: Option<u64>,
    /// A special boolean flag for showing metadata. If set to true, each document entry in the returned JSON will contain a "metadata" field.
    /// This flag only takes effect when file is set.
    pub return_metadata: Option<bool>,
    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    pub user: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub data: Vec<Data>,
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub document: i64,
    pub object: String,
    pub score: f64,
}

impl Request {
    fn url(&self) -> String {
        self.model.url("search")
    }
}
impl Action for Request {
    fn build_request(&self, client: &Client) -> reqwest::RequestBuilder {
        client
            .init_request_data(&self.url())
            .body(serde_json::to_string(self).unwrap())
    }
}
