use serde_derive::Deserialize;
use serde_derive::Serialize;
use typed_builder::TypedBuilder;

use crate::client::Client;
use crate::model::Model;

use super::Action;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Search {
    #[serde(skip)]
    pub model: Model,
    pub documents: Vec<String>,
    pub query: String,
}
impl Action for Search {
    fn build_request(&self, client: &Client) -> reqwest::RequestBuilder {
        client
            .request_client()
            .post(self.model.engine_id())
            .body(serde_json::to_string(self).unwrap())
    }
}
