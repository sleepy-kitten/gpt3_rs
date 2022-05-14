use crate::api::{Action, Auth};
use crate::OPENAI_URL;

use super::{Answers, Classifications, FineTuning, Search};

/// # OpenAi documentation
///
/// Returns the contents of the specified file.
///
/// This differs from [`files::content`][crate::api::files::content], because [`files::content_checked`][crate::api::files::content_checked] will make 2 requests instead of 1.
///
/// The first one will get the metadata of the file, and the second one will get the content and attempt to deserialize it according to the metadata.
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
            .auth(client.gpt_token())
    }
}
#[derive(Debug, Clone)]
pub enum Response {
    Search(super::File<Search>),
    Answers(super::File<Answers>),
    FineTuning(super::File<FineTuning>),
    Classifications(super::File<Classifications>),
}
