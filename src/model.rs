use serde::{Deserialize, Serialize};

use crate::OPENAI_URL;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    /// # Good at
    ///  **Parsing text**, **simple classification**, **address correction**, **keywords**
    /// # description
    /// Capable of very simple tasks, usually the fastest model in the GPT-3 series, and lowest cost.
    /// Ada is usually the fastest model and can perform tasks like parsing text, address correction and certain kinds of classification tasks that don’t require too much nuance. 
    /// Ada’s performance can often be improved by providing more context.
    Ada,
    /// # Good at
    /// **Moderate classification**,**semantic search classification**
    /// # description
    /// Capable of straightforward tasks, very fast, and lower cost.
    ///
    /// Babbage can perform straightforward tasks like simple classification.
    /// It’s also quite capable when it comes to Semantic Search ranking how well documents match up with search queries.
    Babbage,
    /// # Good at
    /// **Language translation**, **complex classification**, **text sentiment**, **summarization**
    /// # description
    /// Very capable, but faster and lower cost than Davinci.
    ///
    /// Curie is extremely powerful, yet very fast. While Davinci is stronger when it comes to analyzing complicated text, Curie is quite capable for many nuanced tasks like sentiment classification and summarization. Curie is also quite good at answering questions and performing Q&A and as a general service chatbot.
    Curie,
    /// # Good at
    /// **Complex intent**, **cause and effect**, **summarization for audience**
    /// # Description
    /// Most capable GPT-3 model. Can do any task the other models can do, often with less context.
    /// In addition to responding to prompts, also supports inserting completions within text.
    ///
    /// Davinci is the most capable engine and can perform any task the other models can perform and often with less instruction.
    /// For applications requiring a lot of understanding of the content, like summarization for a specific audience and creative content generation, Davinci is going to produce the best results.
    /// These increased capabilities require more compute resources, so Davinci costs more per API call and is not as fast as the other engines.
    /// Another area where Davinci shines is in understanding the intent of text.
    /// Davinci is quite good at solving many kinds of logic problems and explaining the motives of characters.
    /// Davinci has been able to solve some of the most challenging AI problems involving cause and effect.
    Davinci,
}

impl Model {
    crate fn url(&self, action: &str) -> String {
        (match self {
            Model::Ada => format!("{OPENAI_URL}/engines/text-ada-001/"),
            Model::Babbage => format!("{OPENAI_URL}/engines/text-babbage-001"),
            Model::Curie => format!("{OPENAI_URL}/engines/text-curie-001"),
            Model::Davinci => format!("{OPENAI_URL}/engines/text-davinci-002"),
        }) + action
    }
}
