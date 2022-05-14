use crate::api::{Action, Auth};
use crate::client::NormalRequest;
use crate::prelude::Purpose;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

use super::FilePurpose;

/// # OpenAi documentation
/// 
/// Upload a file that contains document(s) to be used across various endpoints/features. 
/// Currently, the size of all the files uploaded by one organization can be up to 1 GB. 
/// Please contact us if you need to increase the storage limit.
#[derive(Debug, Clone, PartialEq)]
pub struct Request<T>
where
    T: FilePurpose + Serialize,
{
    /// The file to upload
    pub file: T,
}
impl<T> Request<T>
where
    T: FilePurpose + Serialize,
{
    pub fn new(file: T) -> Self {
        Request { file }
    }
}
#[derive(Debug, Clone, Serialize)]
struct RequestInternal<'a, T>
where
    T: FilePurpose + Serialize,
{
    file: &'a T,
    purpose: Purpose,
}
impl<'a, T> From<&'a Request<T>> for RequestInternal<'a, T>
where
    T: FilePurpose + Serialize,
{
    fn from(request: &'a Request<T>) -> Self {
        RequestInternal {
            file: &request.file,
            purpose: request.file.purpose(),
        }
    }
}
impl<T> Action for Request<T>
where
    T: Serialize + FilePurpose,
{
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .post(format!("{OPENAI_URL}/files/"))
            .auth(client.gpt_token())
            .json(&RequestInternal::from(self))
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
impl<'a, T> NormalRequest for RequestInternal<'a, T> where T: FilePurpose + Serialize {}
