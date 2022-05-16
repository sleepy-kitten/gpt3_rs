use crate::api::files;
use crate::api::BuildRequest;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

/// # OpenAi documentation
///
/// Returns a list of files that belong to the user's organization.
#[derive(Debug, Clone, PartialEq)]
pub struct Request;

impl BuildRequest for Request {

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files"))
            .bearer_auth(client.gpt_token())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// List of files and metadata uploaded to the storage
    pub data: Vec<files::metadata::Response>,
    /// Action of the request
    pub object: String,
}
#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl crate::client::Request for Request {
    type Response = Response;
}