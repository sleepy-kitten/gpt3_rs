use reqwest::RequestBuilder;

use crate::action::Action;

pub struct Client {
    request_client: reqwest::Client,
    gpt_token: String,
}
impl Client {
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
        &self.request_client
    }
    pub fn add_request_data(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder
            .header("Content-Type", "application/json")
            .bearer_auth(self.gpt_token())
    }
}
