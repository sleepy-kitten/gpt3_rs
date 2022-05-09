use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::model::Model;

use super::Url;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Request {
    #[serde(skip)]
    pub model: Model,
    /// The input text to use as a starting point for the edit.
    pub input: Option<String>,
    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,
    /// What sampling temperature to use. Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    /// We generally recommend altering this or top_p but not both.
    pub temperature: Option<f64>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    pub top_p: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: usize,
}
impl Url for Request {
    fn url(&self) -> String {
        self.model.url("edits")
    }
}
