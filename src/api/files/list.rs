use serde::{Deserialize, Serialize};
use crate::api::{Action, Auth};
use crate::OPENAI_URL;

struct Request;

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
    pub data: Vec<Data>,
    /// Action of the request
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// The file id used to identify the file
    pub id: String,
    /// The object of the request
    pub object: String,
    /// The file size in bytes
    pub bytes: u64,
    /// The time the file was uploaded
    pub created_at: u64,
    /// The name of the file
    pub filename: String,
    /// The purpose of the file
    pub purpose: String,
}
