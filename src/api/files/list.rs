use crate::api::files;
use crate::api::{Action, Auth};
use crate::OPENAI_URL;
use crate::client::NormalRequest;
use serde::{Deserialize, Serialize};

pub struct Request;

impl Action for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files"))
            .auth(client.gpt_token())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// List of files and metadata uploaded to the storage
    pub data: Vec<files::metadata::Response>,
    /// Action of the request
    pub object: String,
}
impl NormalRequest for Request {}
