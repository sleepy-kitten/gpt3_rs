use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::Purpose;

pub mod content;
pub mod content_checked;
pub mod delete;
pub mod list;
pub mod metadata;
pub mod upload;

pub trait FilePurpose {
    fn purpose(&self) -> Purpose;
}
pub trait ValidFile {}
/// A file with the purpose "search"
///
/// has a text field and arbitrary metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search<T = ()> {
    pub text: String,
    pub metadata: T,
}
/// A file with the purpose "answers"
///
/// has a text field and arbitrary metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answers<T = ()> {
    pub text: String,
    pub metadata: T,
}
/// A file with the purpose "classifications"
///
/// has a text field, a label field and arbitrary metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classifications<T = ()> {
    pub text: String,
    pub label: String,
    pub metadata: T,
}
/// A file with the purpose "fine-tune"
///
/// has a prompt and a completion field
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

#[derive(Debug, Clone, Deserialize)]
pub struct File<T> {
    #[serde(skip)]
    pub name: String,
    pub lines: Vec<T>,
}

impl<T> File<T> {
    pub fn new(name: String, lines: Vec<T>) -> Self {
        File { name, lines }
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
impl<T> ValidFile for File<Search<T>> {}
impl<T> ValidFile for File<Answers<T>> {}
impl<T> ValidFile for File<Classifications<T>> {}
impl ValidFile for File<FineTuning> {}
impl ValidFile for Raw {}
