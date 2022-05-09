use serde_derive::Deserialize;
use serde_derive::Serialize;
use typed_builder::TypedBuilder;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Edit {
    pub input: String,
    pub instruction: String,
}
