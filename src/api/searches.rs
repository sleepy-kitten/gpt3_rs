use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::model::Model;

use super::RequestInfo;
/// Given a query and a set of documents or labels, the model ranks each document based on its semantic similarity to the provided query.
///
/// Related guide: [Search](https://beta.openai.com/docs/guides/search)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder, )]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Request {
    #[serde(skip)]
    #[builder(!default, setter(!strip_option))]
    pub model: Model,
    /// Query to search against the documents.
    #[builder(!default, setter(!strip_option))]
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
    /// The ansers returned by this request
    pub data: Vec<Data>,
    /// The requested action
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// The Document index a match was found in
    pub document: usize,
    /// The requested action
    pub object: String,
    /// The score / certainty that this is a match
    pub score: f64,
}

impl RequestInfo for Request {
    type Response = Response;
    fn url(&self) -> String {
        self.model.url("search")
    }
}
