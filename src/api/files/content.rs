use crate::api::Action;
use crate::client::NormalRequest;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

/// # OpenAi documentation
///
/// Returns the contents of the specified file
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
            .get(format!("{OPENAI_URL}/files/{}/content", self.file_id))
            .bearer_auth(client.gpt_token())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub content: String,
}
impl NormalRequest for Request {}
