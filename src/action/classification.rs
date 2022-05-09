use serde_derive::Deserialize;
use serde_derive::Serialize;
use typed_builder::TypedBuilder;

use crate::model::Model;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Classification {
    pub examples: Vec<Vec<String>>,
    pub labels: Vec<String>,
    pub query: String,
    pub search_model: Model,
    pub model: Model,
}
