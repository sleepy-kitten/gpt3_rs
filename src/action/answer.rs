use serde_derive::Deserialize;
use serde_derive::Serialize;
use typed_builder::TypedBuilder;

use crate::model::Model;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Answer {
    pub documents: Vec<String>,
    pub question: String,
    pub search_model: Model,
    pub model: Model,
    pub examples_context: String,
    pub examples: Vec<Vec<String>>,
    #[builder(default = 128)]
    pub max_tokens: i64,
    pub stop: Vec<String>,
}