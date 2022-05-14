use crate::api::Action;
#[cfg(not(feature = "blocking"))]
use async_trait::async_trait;
use serde::de::DeserializeOwned;

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
pub trait NormalRequest {}

/// Sends the request to the OpenAi api
///
/// [`request`][Request::request()] returns a deserialzed version of the response,
/// while [`request_raw`][Request::request_raw()] returns a String
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
/// let response = request.request(&client).await.unwrap();
/// ```
#[cfg_attr(not(feature = "blocking"), async_trait)]
pub trait Request {
    type Response;
    #[cfg(feature = "blocking")]
    fn request(&self, client: &Client) -> reqwest::Result<Self::Response>;
    #[cfg(feature = "blocking")]
    fn request_raw(&self, client: &Client) -> reqwest::Result<String>;

    #[cfg(not(feature = "blocking"))]
    async fn request(&self, client: &Client) -> reqwest::Result<Self::Response>;
    #[cfg(not(feature = "blocking"))]
    async fn request_raw(&self, client: &Client) -> reqwest::Result<String>;
}

#[cfg(feature = "blocking")]
impl<T> Request for T
where
    T: Action + NormalRequest,
    T::Response: DeserializeOwned,
{
    type Response = T::Response;

    fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let response = self.build_request(client).send()?;
        let json = response.json()?;
        Ok(json)
    }

    fn request_raw(&self, client: &Client) -> reqwest::Result<String> {
        let response = self.build_request(client).send()?;
        let json = response.text()?;
        Ok(json)
    }
}

#[cfg(feature = "blocking")]
impl Request for crate::api::files::content_checked::Request {
    type Response = crate::api::files::content_checked::Response;

    fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let metadata = crate::api::files::metadata::Request {
            file_id: self.file_id.clone(),
        }
        .request(client)?;
        let response = self.build_request(client).send()?;
        let file = match metadata.purpose {
            crate::prelude::Purpose::Search => Self::Response::Search(response.json()?),

            crate::prelude::Purpose::Answers => Self::Response::Answers(response.json()?),
            crate::prelude::Purpose::Classifications => {
                Self::Response::Classifications(response.json()?)
            }
            crate::prelude::Purpose::FineTuning => Self::Response::FineTuning(response.json()?),
        };
        Ok(file)
    }
    fn request_raw(&self, client: &Client) -> reqwest::Result<String> {
        crate::api::files::content::Request {
            file_id: self.file_id.clone(),
        }
        .request_raw(client)
    }
}
#[cfg(not(feature = "blocking"))]
#[async_trait]
impl<T> Request for T
where
    T: Action + NormalRequest + Sync,
    T::Response: DeserializeOwned,
{
    type Response = T::Response;

    async fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let response = self.build_request(client).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
    async fn request_raw(&self, client: &Client) -> reqwest::Result<String> {
        let response = self.build_request(client).send().await?;
        let json = response.text().await?;
        Ok(json)
    }
}

#[cfg(not(feature = "blocking"))]
#[async_trait]
impl Request for crate::api::files::content_checked::Request {
    type Response = crate::api::files::content_checked::Response;

    async fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let metadata = crate::api::files::metadata::Request {
            file_id: self.file_id.clone(),
        }
        .request(client)
        .await?;

        let response = self.build_request(client).send().await?;

        let file = match metadata.purpose {
            crate::prelude::Purpose::Search => Self::Response::Search(response.json().await?),

            crate::prelude::Purpose::Answers => Self::Response::Answers(response.json().await?),
            crate::prelude::Purpose::Classifications => {
                Self::Response::Classifications(response.json().await?)
            }
            crate::prelude::Purpose::FineTuning => {
                Self::Response::FineTuning(response.json().await?)
            }
        };
        Ok(file)
    }
    async fn request_raw(&self, client: &Client) -> reqwest::Result<String> {
        crate::api::files::content::Request {
            file_id: self.file_id.clone(),
        }
        .request_raw(client)
        .await
    }
}
