use serde::{
    de::{DeserializeOwned, IntoDeserializer},
    Deserialize, Serialize,
};

use super::Purpose;

pub mod content;
pub mod delete;
pub mod list;
pub mod metadata;
pub mod upload;

pub trait FilePurpose {
    fn purpose(&self) -> Purpose;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search<T = ()> {
    pub text: String,
    pub metadata: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answers<T = ()> {
    pub text: String,
    pub metadata: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classifications<T = ()> {
    pub text: String,
    pub label: String,
    pub metadata: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuning {
    pub prompt: String,
    pub completion: String,
}
pub struct Raw {
    pub data: String,
    pub purpose: Purpose,
}

impl<T: Serialize + DeserializeOwned> FilePurpose for Search<T> {
    fn purpose(&self) -> Purpose {
        Purpose::Search
    }
}
impl<T: Serialize> FilePurpose for Answers<T> {
    fn purpose(&self) -> Purpose {
        Purpose::Answers
    }
}

impl<T: Serialize> FilePurpose for Classifications<T> {
    fn purpose(&self) -> Purpose {
        Purpose::Classifications
    }
}

impl FilePurpose for FineTuning {
    fn purpose(&self) -> Purpose {
        Purpose::FineTuning
    }
}
impl FilePurpose for Raw {
    fn purpose(&self) -> Purpose {
        self.purpose
    }
}

impl<T> TryFrom<content::Response> for Search<T>
where
    T: DeserializeOwned,
{
    type Error = serde_json::Error;

    fn try_from(value: content::Response) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.content)
    }
}
impl<T> TryFrom<content::Response> for Answers<T>
where
    T: DeserializeOwned,
{
    type Error = serde_json::Error;

    fn try_from(value: content::Response) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.content)
    }
}
impl<T> TryFrom<content::Response> for Classifications<T>
where
    T: DeserializeOwned,
{
    type Error = serde_json::Error;

    fn try_from(value: content::Response) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.content)
    }
}
impl TryFrom<content::Response> for FineTuning {
    type Error = serde_json::Error;

    fn try_from(value: content::Response) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.content)
    }
}
