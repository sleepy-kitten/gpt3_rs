use crate::api::BuildRequest;
use crate::client::NormalRequest;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

/// # OpenAi documentation
///
/// Delete a file.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub file_id: String,
}
impl Request {
    pub fn new(file_id: String) -> Self {
        Request { file_id }
    }
}
impl BuildRequest for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .delete(format!("{OPENAI_URL}/files/{}", self.file_id))
            .bearer_auth(client.gpt_token())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The file id used to identify the file
    pub id: String,
    /// The object of the request
    pub object: String,
    /// Whether the deletion was successful or not
    pub deleted: bool,
}

impl NormalRequest for Request {}
