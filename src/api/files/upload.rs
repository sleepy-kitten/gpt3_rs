use crate::api::BuildRequest;
use crate::client::NormalRequest;
use crate::prelude::Purpose;
use crate::{Form, Part, OPENAI_URL};
use serde::{Deserialize, Serialize};

use super::{File, FilePurpose, ValidFile};

/// # OpenAi documentation
///
/// Upload a file that contains document(s) to be used across various endpoints/features.
/// Currently, the size of all the files uploaded by one organization can be up to 1 GB.
/// Please contact us if you need to increase the storage limit.
#[derive(Debug, Clone)]
pub struct Request<T: ValidFile> {
    /// The file to upload
    pub file: File<T>,
}
impl<T: ValidFile> Request<T> {
    pub fn new(file: File<T>) -> Self {
        Request { file }
    }
}
impl<T> BuildRequest for Request<T>
where
    File<T>: FilePurpose,
    T: Serialize + ValidFile,
{
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        let content = self
            .file
            .lines
            .iter()
            .map(serde_json::to_string)
            .map(Result::unwrap)
            .map(|string| string + "\n")
            .collect::<String>();

        let form = Form::new()
            .part("purpose", Part::text(self.file.purpose().to_string()))
            .part(
                "file",
                Part::text(content).file_name(self.file.name.clone()),
            );

        client
            .reqwest_client()
            .post(format!("{OPENAI_URL}/files"))
            .bearer_auth(client.gpt_token())
            .multipart(form)
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
impl<T: ValidFile> NormalRequest for Request<T> {}
