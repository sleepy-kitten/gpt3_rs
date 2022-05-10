//! Create completions for a prompt
//! # Builder
//! Use the [`completions::Builder`][struct@Builder] to construct a [`completions::Request`][Request] struct
use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{model::Model, into_vec::IntoVec};

use super::RequestInfo;
/// Create completions for a prompt
/// 
/// # OpenAi documentation
/// Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.
/// # Example
/// ```
/// let request = completion::Builder::default()
///     .model(Model::Curie)
///     .prompt("Say this is a test")
///     .max_tokens(5)
///     .temperature(1.0)
///     .top_p(1.0)
///     .n(1)
///     .stop("\n")
///     .build()
///     .unwrap();
/// ```
/// # Required
/// ```rs
/// model
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder_struct_attr(doc = "# Required")]
#[builder_struct_attr(doc = "[`model`][Self::model()]")]
#[builder_struct_attr(doc = "")]
#[builder(name = "Builder")]
pub struct Request {
    #[serde(skip_serializing)]
    pub model: Model,
    /// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.
    /// Note that <|endoftext|> is the document separator that the model sees during training,
    /// so if a prompt is not specified the model will generate as if from the beginning of a new document.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<IntoVec<String>>,
    /// The suffix that comes after a completion of inserted text.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// The maximum number of tokens to generate in the completion.
    /// The token count of your prompt plus max_tokens cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    /// # Default
    /// Defaults to 16
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// What sampling temperature to use. Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    /// We generally recommend altering this or top_p but not both.
    /// # Default
    /// Defaults to 1.0
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    /// # Default
    /// Defaults to 1.0
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// How many completions to generate for each prompt.
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for max_tokens and stop.
    /// # Default
    /// Defaults to 1
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Whether to stream back partial progress.
    /// If set, tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message.
    /// # Default
    /// Defaults to false
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens.
    /// For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response.
    /// The maximum value for logprobs is 5. If you need more than this, please contact support@openai.com and describe your use case.
    /// # Default
    /// Defaults to none
    #[builder(default, setter(strip_option))]
    pub logprobs: Option<u8>,
    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<IntoVec<String>>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties](https://beta.openai.com/docs/api-reference/parameter-details)
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties](https://beta.openai.com/docs/api-reference/parameter-details)
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Generates best_of completions server-side and returns the "best" (the one with the lowest log probability per token). Results cannot be streamed.
    /// When used with n, best_of controls the number of candidate completions and n specifies how many to return – best_of must be greater than n.
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for max_tokens and stop.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u8>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100.
    /// You can use this tokenizer tool (which works for both GPT-2 and GPT-3) to convert text to token IDs.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    /// As an example, you can pass {"50256": -100} to prevent the <|endoftext|> token from being generated.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i8>>,
    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
/// A response corresponding to a [`Request`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// ?
    pub id: String,
    /// The requested action
    pub object: String,
    /// The creation Time of the request
    pub created: u64,
    /// The model used to create the completion
    pub model: String,
    /// The answers created by this request
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// The text generated by the model
    pub text: String,
    /// The index of this choice
    pub index: usize,
    /// A list of the n most likely tokens
    pub logprobs: Option<Vec<String>>,
    /// reason why the model finished
    pub finish_reason: String,
}
impl RequestInfo for Request {
    type Response = Response;
    fn url(&self) -> String {
        self.model.url("/completions")
    }
}
