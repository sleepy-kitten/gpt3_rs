use serde::{Deserialize, Serialize};
use crate::api::{Action, Auth};
use crate::OPENAI_URL;
use crate::client::NormalRequest;

/// # OpenAi documentation
/// 
/// Returns the contents of the specified file
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub file_id: String,
}

impl Action for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files/{}/content", self.file_id))
            .auth(client.gpt_token())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub content: String,
}
impl NormalRequest for Request {}
