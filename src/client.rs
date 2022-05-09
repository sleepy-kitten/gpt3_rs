use std::error::Error;

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::api::Action;

pub struct Client {
    reqwest_client: reqwest::Client,
    gpt_token: String,
}
impl Client {
    pub fn new(token: String) -> Self {
        Client {
            reqwest_client: reqwest::Client::new(),
            gpt_token: token,
        }
    }
    pub async fn request<T>(&self, request: T) -> Result<T::Response, Box<dyn Error>>
    where
        T: Action,
        T::Response: DeserializeOwned,
    {
        let response = request.build_request(self).send().await?;
        let text = response.text().await?;
        let deserialized: T::Response = serde_json::from_str(&text)?;
        Ok(deserialized)
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
    crate fn init_post(&self, url: &str) -> RequestBuilder {
        let builder = self.reqwest_client.post(url);
        self.init_request(builder)
    }
    crate fn init_request(&self, builder: RequestBuilder) -> RequestBuilder {
        builder
            .header("Content-Type", "application/json")
            .bearer_auth(self.gpt_token())
    }
}
