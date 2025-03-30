use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TranslatedWordData {
    definitions: Vec<TranslatedDefinition>,
}

#[derive(Debug, Serialize)]
pub struct TranslatedDefinition {
    pub meaning: Vec<String>,
}

//#[derive(Debug, Serialize)] pub struct TranslatedMeaning {
//pub description: String,
//}

impl TranslatedWordData {
    pub fn new(definitions: Vec<Vec<String>>) -> Self {
        Self {
            definitions: definitions
                .into_iter()
                .map(|tr_meanings| TranslatedDefinition {
                    meaning: tr_meanings,
                })
                .collect(),
        }
    }
}
