use serde_derive::{Deserialize, Serialize};

use crate::OPENAI_URL;

pub struct Ada;
pub struct Babbage;
pub struct Curie;
pub struct Davinci;

impl_const_str!(Ada, "text-ada-001");
impl_const_str!(Babbage, "text-babbage-001");
impl_const_str!(Curie, "text-curie-001");
impl_const_str!(Davinci, "text-davinci-002");

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    Ada,
    Babbage,
    #[default]
    Curie,
    Davinci,
}

impl Model {
    crate fn engine_id(&self) -> &'static str {
        match self {
            Model::Ada => const_format::formatcp!("{OPENAI_URL}/text-ada-001"),
            Model::Babbage => const_format::formatcp!("{OPENAI_URL}/text-babbage-001"),
            Model::Curie => const_format::formatcp!("{OPENAI_URL}/text-curie-001"),
            Model::Davinci => const_format::formatcp!("{OPENAI_URL}/text-davinci-002"),
        }
    }
}
