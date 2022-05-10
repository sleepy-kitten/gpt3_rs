use crate::api::{Action, Auth};
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

use super::File;

#[derive(Debug, Clone, Serialize)]
struct Request<T>
where
    T: Serialize,
{
    file: File<T>,
    purpose: String,
}

impl<T> Action for Request<T>
where
    T: Serialize,
{
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> reqwest::RequestBuilder {
        client
            .reqwest_client()
            .post(format!("{OPENAI_URL}/files/"))
            .body(serde_json::to_string(self).unwrap())
            .auth(client.gpt_token())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub purpose: String,
}
