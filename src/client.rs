use reqwest::RequestBuilder;

use crate::{api::Action, model::Model};

pub struct Client {
    reqwest_client: reqwest::Client,
    gpt_token: String,
    default_model: Model,
}
impl Client {
    pub fn new(token: String) -> Self {
        Client {
            reqwest_client: reqwest::Client::new(),
            gpt_token: token,
            default_model: Model::Curie,
        }
    }
    pub async fn request<T>(&self, action: T) -> reqwest::Result<reqwest::Response>
    where
        T: Action,
    {
        action.build_request(self).send().await
    }

    /// Get a reference to the client's gpt token.
    #[must_use]
    pub fn gpt_token(&self) -> &str {
        self.gpt_token.as_ref()
    }

    /// Get a reference to the client's request client.
    #[must_use]
    pub fn request_client(&self) -> &reqwest::Client {
        &self.reqwest_client
    }
    crate fn init_request_data(&self, url: &str) -> RequestBuilder {
        self.reqwest_client
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.gpt_token())
    }
}
