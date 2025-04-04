use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WordData {
    pub word: String,
    pub definitions: Vec<Definition>,
    pub pronunciation_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Definition {
    pub pos: String,
    pub meaning: Vec<Meaning>,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct Meaning {
    pub description: String,
    pub examples: Vec<String>,
}

impl WordData {
    pub fn new(
        word: String,
        pos: String,
        meaning: Vec<Meaning>,
        pronunciation_url: Option<String>,
        source: String,
    ) -> Self {
        WordData {
            word,
            definitions: vec![Definition {
                pos,
                meaning,
                source,
            }],
            pronunciation_url,
        }
    }

    pub fn empty(word: String) -> Self {
        WordData {
            word,
            definitions: Vec::new(),
            pronunciation_url: None,
        }
    }

    pub fn add_definition(
        &mut self,
        word: String,
        defs: Vec<Definition>,
        pronunciation_url: Option<String>,
    ) {
        if self.word != word {
            self.word.push_str(&format!(" / {}", word));
        }
        self.definitions.extend(defs);
        self.pronunciation_url = pronunciation_url;
    }
}
