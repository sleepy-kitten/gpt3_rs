use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Completion {
    pub prompt: String,
    #[builder(default = 128)]
    pub max_tokens: i64,
    #[builder(default = 1.0)]
    pub temperature: f64,
    #[builder(default = 1.0)]
    pub top_p: f64,
    #[builder(default = 1)]
    pub n: i64,
    #[builder(default = false)]
    pub stream: bool,
    #[builder(default = Value::Null)]
    pub logprobs: Value,
    pub stop: String,
}
