use crate::api::{Action};
use crate::client::NormalRequest;
use crate::prelude::Purpose;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

/// # OpenAi documentation
///
/// Returns information about a specific file.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub file_id: String,
}
impl Request {
    pub fn new(file_id: String) -> Self {
        Request { file_id }
    }
}
impl Action for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files/{}", self.file_id))
            .bearer_auth(client.gpt_token())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The file id used to identify the file
    pub id: String,
    /// The object of the request
    pub object: String,
    /// The size of the file in bytes
    pub bytes: u64,
    /// The time the file was uploaded
    pub created_at: u64,
    /// The name of the file
    pub filename: String,
    /// The purpose of the file
    pub purpose: Purpose,
}
impl NormalRequest for Request {}
