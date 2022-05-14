use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::Purpose;

pub mod content;
pub mod content_checked;
pub mod delete;
pub mod list;
pub mod metadata;
pub mod upload;

/// A file with the purpose "search"
///
/// has a text field
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
#[derive(Debug, Clone)]
pub struct Raw {
    pub data: String,
    pub purpose: Purpose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File<T> {
    pub lines: Vec<T>,
}

impl<T> File<T> {
    pub fn new(lines: Vec<T>) -> Self {
        File { lines }
    }
}

impl<T: Serialize + DeserializeOwned> FilePurpose for File<Search<T>> {
    fn purpose(&self) -> Purpose {
        Purpose::Search
    }
}
impl<T: Serialize> FilePurpose for File<Answers<T>> {
    fn purpose(&self) -> Purpose {
        Purpose::Answers
    }
}

impl<T: Serialize> FilePurpose for File<Classifications<T>> {
    fn purpose(&self) -> Purpose {
        Purpose::Classifications
    }
}

impl FilePurpose for File<FineTuning> {
    fn purpose(&self) -> Purpose {
        Purpose::FineTuning
    }
}
impl FilePurpose for Raw {
    fn purpose(&self) -> Purpose {
        self.purpose
    }
}
