use crate::api::{Action, Auth};
use crate::OPENAI_URL;

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
#[derive(Debug, Clone)]
pub enum Response {
    Search(super::Search),
    Answers(super::Answers),
    FineTuning(super::FineTuning),
    Classifications(super::Classifications),
}
