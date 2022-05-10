//! Answers questions from provided context
//! # Builder
//! Use the [`answers::Builder`][struct@Builder] to construct an [`answers::Request`][Request] struct
use std::collections::HashMap;

use crate::{model::Model, into_vec::IntoVec};
use crate::OPENAI_URL;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::RequestInfo;
/// Answers questions from provided context
/// 
/// # OpenAi documentation
/// Answers the specified question using the provided documents and examples.
///
/// The endpoint first searches over provided documents or files to find relevant context.
/// The relevant context is combined with the provided examples and question to create the prompt for completion.
///
/// # Example
/// ```
/// let request = answers::Builder::default()
///     .model(Model::Curie)
///     .search_model(Model::Ada)
///     .question("which puppy is happy?")
///     .documents(vec!["Puppy A is happy".into(),"Puppy B is sad.".into()])
///     .example_context("In 2017, U.S. life expectancy was 78.6 years.")
///     .examples(vec![
///         vec![
///             "What is human life expectancy in the United States?".into(),
///             "78 years.".into()
///         ]
///     ])
///     .max_tokens(5)
///     .stop(vec!["\n".into(), "<|endoftext|>".into()])
///     .build()
///     .unwrap();
/// ```
/// # Required
/// ```
/// model, question, examples, examples_context
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder_struct_attr(doc = "# Required")]
#[builder_struct_attr(doc = "[`model`][Self::model()]")]
#[builder_struct_attr(doc = "[`question`][Self::question()]")]
#[builder_struct_attr(doc = "[`examples`][Self::examples()]")]
#[builder_struct_attr(doc = "[`examples_context`][Self::examples_context()]")]
#[builder_struct_attr(doc = "")]
#[builder(name = "Builder")]
pub struct Request {
    /// ID of the engine to use for completion. You can select one of ada, babbage, curie, or davinci.
    //#[builder(!default, setter(!strip_option))]
    pub model: Model,
    /// Question to get answered.
    #[builder(setter(into))]
    pub question: String,
    /// List of (question, answer) pairs that will help steer the model towards the tone and answer format you'd like.
    /// We recommend adding 2 to 3 examples.
    //#[builder(!default, setter(!strip_option))]
    #[builder(setter(into))]
    pub examples: IntoVec<Vec<String>>,
    /// A text snippet containing the contextual information used to generate the answers for the examples you provide.
    //#[builder(!default, setter(!strip_option))]
    #[builder(setter(into))]
    pub examples_context: String,
    /// List of documents from which the answer for the input question should be derived.
    /// If this is an empty list, the question will be answered based on the question-answer examples.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<IntoVec<String>>,
    /// The ID of an uploaded file that contains documents to search over. See upload file for how to upload a file of the desired format and purpose.
    /// You should specify either documents or a file, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// ID of the engine to use for Search. You can select one of ada, babbage, curie, or davinci.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_model: Option<Model>,
    /// The maximum number of documents to be ranked by Search when using file. Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rerank: Option<u64>,
    /// What sampling temperature to use. Higher values mean the model will take more risks and value 0 (argmax sampling) works better for scenarios with a well-defined answer.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens.
    /// For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response.
    /// The maximum value for logprobs is 5. If you need more than this, please contact support@openai.com and describe your use case.
    /// When logprobs is set, completion will be automatically added into expand to get the logprobs.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,
    /// The maximum number of tokens allowed for the generated answer
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<IntoVec<String>>,
    /// How many answers to generate for each question.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u16>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100.
    /// You can use this tokenizer tool (which works for both GPT-2 and GPT-3) to convert text to token IDs.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    /// As an example, you can pass {"50256": -100} to prevent the <|endoftext|> token from being generated.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bits: Option<HashMap<String, i8>>,
    /// A special boolean flag for showing metadata.
    /// If set to true, each document entry in the returned JSON will contain a "metadata" field.
    /// This flag only takes effect when file is set.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    /// If set to true, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion. T
    /// his is mainly useful for debugging purposes.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_prompt: Option<bool>,
    /// If an object name is in the list, we provide the full information of the object;
    /// otherwise, we only provide the object ID. Currently we support completion and file objects for expansion.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<IntoVec<String>>,
    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
/// A response corresponding to a [`Request`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// the answers generated by the request
    pub answers: Vec<String>,
    /// ?
    pub completion: String,
    /// the model used for the request
    pub model: String,
    /// the object / action of the request
    pub object: String,
    /// the model used to search through the text
    pub search_model: String,
    /// documents sent in the request
    pub selected_documents: Vec<SelectedDocument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedDocument {
    /// document index
    pub document: usize,
    /// document content
    pub text: String,
}

impl RequestInfo for Request {
    type Response = Response;
    fn url(&self) -> String {
        format!("{OPENAI_URL}/answers")
    }
}
