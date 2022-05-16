use async_trait::async_trait;

use crate::OPENAI_URL;
use crate::{api::BuildRequest, Client};

use super::{Answers, Classifications, File, FineTuning, Search};

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
impl BuildRequest for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files/{}/content", self.file_id))
            .bearer_auth(client.gpt_token())
    }
}
#[derive(Debug, Clone)]
pub enum Response {
    Search(super::File<Search>),
    Answers(super::File<Answers>),
    FineTuning(super::File<FineTuning>),
    Classifications(super::File<Classifications>),
}

#[cfg(not(feature = "blocking"))]
#[async_trait]
impl crate::client::Request for crate::api::files::content_checked::Request {
    type Response = crate::api::files::content_checked::Response;

    async fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let metadata = crate::api::files::metadata::Request::new(self.file_id.clone())
            .request(client)
            .await?;

        let response = crate::api::files::content::Request::new(self.file_id.clone())
            .request_raw(client)
            .await?;

        let text = dbg!(response);
        let iter = text.lines();

        let file = match metadata.purpose {
            crate::prelude::Purpose::Search => Self::Response::Search(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Search>>(),
            )),
            crate::prelude::Purpose::Answers => Self::Response::Answers(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Answers>>(),
            )),
            crate::prelude::Purpose::Classifications => Self::Response::Classifications(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Classifications>>(),
            )),
            crate::prelude::Purpose::FineTuning => Self::Response::FineTuning(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<FineTuning>>(),
            )),
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
#[cfg(feature = "blocking")]
impl crate::client::Request for crate::api::files::content_checked::Request {
    type Response = crate::api::files::content_checked::Response;

    fn request(&self, client: &Client) -> reqwest::Result<Self::Response> {
        let metadata =
            crate::api::files::metadata::Request::new(self.file_id.clone()).request(client)?;

        let response =
            crate::api::files::content::Request::new(self.file_id.clone()).request_raw(client)?;
        let text = dbg!(response);
        let iter = text.lines();

        let file = match metadata.purpose {
            crate::prelude::Purpose::Search => Self::Response::Search(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Search>>(),
            )),
            crate::prelude::Purpose::Answers => Self::Response::Answers(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Answers>>(),
            )),
            crate::prelude::Purpose::Classifications => Self::Response::Classifications(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<Classifications>>(),
            )),
            crate::prelude::Purpose::FineTuning => Self::Response::FineTuning(File::new(
                metadata.filename,
                iter.map(serde_json::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<FineTuning>>(),
            )),
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
