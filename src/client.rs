use serde::de::DeserializeOwned;

use crate::api::Action;
#[cfg(not(feature = "blocking"))]
type RequestClient = reqwest::Client;
#[cfg(feature = "blocking")]
type RequestClient = reqwest::blocking::Client;

/// A client for interacting with the OpenAi api
/// # Example
/// ```ignore
/// let client = Client::new(token);
/// let request = completions::Builder::default()
///     .model(Model::Babbage)
///     .prompt("what is 1 + 2?".into())
///     .build()
///     .unwrap();
///
/// let response = client.request(request).await.unwrap();
/// ```
pub struct Client {
    reqwest_client: RequestClient,
    gpt_token: String,
}
impl Client {
    /// Creates a new client to send requests from
    /// # Example
    /// ```ignore
    /// let token = std::env::var("GPT_API_TOKEN").unwrap();
    /// let client = Client::new(token);
    /// ```
    pub fn new(token: String) -> Self {
        Client {
            reqwest_client: RequestClient::new(),
            gpt_token: token,
        }
    }
    /// Creates a request from the passed struct
    ///
    /// # Usage
    /// using a builder to build the request struct is strongly advised
    /// # Example
    /// ```ignore
    /// let request = completions::Builder::default()
    ///     .model(Model::Babbage)
    ///     .prompt("what is 1 + 2?".into())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = client.request(request).await.unwrap();
    /// ```
    #[cfg(not(feature = "blocking"))]
    pub async fn request<T>(&self, request: &T) -> reqwest::Result<T::Response>
    where
        T: Action,
        T::Response: DeserializeOwned,
    {
        let response = request.build_request(self).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
    #[cfg(feature = "blocking")]
    pub fn request<T>(&self, request: &T) -> reqwest::Result<T::Response>
    where
        T: Action,
        T::Response: DeserializeOwned,
    {
        let response = request.build_request(self).send()?;
        let json = response.json()?;
        Ok(json)
    }
    /// Get a reference to the client's gpt token.
    #[must_use]
    pub fn gpt_token(&self) -> &str {
        self.gpt_token.as_ref()
    }
    /// Get a reference to the client's request client.
    #[must_use]
    pub fn reqwest_client(&self) -> &RequestClient {
        &self.reqwest_client
    }
}
