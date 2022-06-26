//! Rank provided documents based off of a query
//!
//! # Builder
//! Use the [`searches::Builder`][struct@Builder] to construct a [`searches::Request`][Request] struct
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{into_vec::IntoVec, model::Model};

use super::RequestInfo;
/// Rank provided documents based off of a query
///
/// # OpenAi documentation
/// Given a query and a set of documents or labels, the model ranks each document based on its semantic similarity to the provided query.
///
/// Related guide: [Search](https://beta.openai.com/docs/guides/search)
/// # Example
/// ```ignore
/// let request = searches::Builder::default()
///     .model(Model::Curie)
///     .documents(&[
///         "White house",
///         "hospital",
///         "school",
///     ])
///     .query("the president")
///     .build()
///     .unwrap();
/// ```
/// # Required
///
/// `model`, `query
///
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder_struct_attr(doc = "# Required")]
#[builder_struct_attr(doc = "[`model`][Self::model()]")]
#[builder_struct_attr(doc = "[`query`][Self::query()]")]
#[builder_struct_attr(doc = "")]
#[builder(name = "Builder")]
pub struct Request {
    #[serde(skip_serializing)]
    pub model: Model,
    /// Query to search against the documents.
    #[builder(setter(into))]
    pub query: String,
    /// Up to 200 documents to search over, provided as a list of strings.
    ///The maximum document length (in tokens) is 2034 minus the number of tokens in the query.
    ///You should specify either documents or a file, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<IntoVec<String>>,
    /// The ID of an uploaded file that contains documents to search over.
    /// You should specify either documents or a file, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The maximum number of documents to be re-ranked and returned by search.
    /// This flag only takes effect when file is set.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rerank: Option<u64>,
    /// A special boolean flag for showing metadata. If set to true, each document entry in the returned JSON will contain a "metadata" field.
    /// This flag only takes effect when file is set.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// A response corresponding to a [`Request`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The requested action
    pub object: Option<String>,
    /// The ansers returned by this request
    pub data: Vec<Data>,
    pub model: Option<String>,
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
    fn url(&self) -> String {
        self.model.url("/search")
    }
}
#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl crate::client::Request for Request {
    type Response = Response;
}
