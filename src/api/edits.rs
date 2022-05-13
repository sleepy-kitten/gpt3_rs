//!Edit text based off of an instruction
//! # Builder
//! Use the [`edits::Builder`][struct@Builder] to construct a [`edits::Request`][Request] struct
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::model::Model;

use super::RequestInfo;
/// Edit text based off of an instruction
/// 
/// # OpenAi documentation
/// Given a prompt and an instruction, the model will return an edited version of the prompt.
/// # Example
/// ```ignore
/// let request = edits::Builder::default()
///     .model(Model::Curie)
///     .input("What day of the wek is it?")
///     .instruction("Fix the spelling mistakes")
///     .build()
///     .unwrap();
/// ```
/// # Required
/// ```ignore
/// model
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder_struct_attr(doc = "# Required")]
#[builder_struct_attr(doc = "[`model`][Self::model()]")]
#[builder_struct_attr(doc = "[`instruction`][Self::instruction()]")]
#[builder_struct_attr(doc = "")]
#[builder(name = "Builder")]
pub struct Request {
    #[serde(skip_serializing)]
    pub model: Model,
    /// The input text to use as a starting point for the edit.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The instruction that tells the model how to edit the prompt.
    #[builder(setter(into))]
    pub instruction: String,
    /// What sampling temperature to use. Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    /// We generally recommend altering this or top_p but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}
/// A response corresponding to a [`Request`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The requested action
    pub object: String,
    /// The creation Time of the request
    pub created: u64,
    /// The choices return by the model
    pub choices: Vec<Choice>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// The text generated by the model
    pub text: String,
    /// The index of this choice
    pub index: usize,
}
impl RequestInfo for Request {
    type Response = Response;
    fn url(&self) -> String {
        self.model.edit_url("/edits")
    }
}
