#[derive(Debug)]
pub struct WordData {
    word: String,
    definitions: Vec<Definition>,
    pronunciation_url: Option<String>,
}

#[derive(Debug)]
pub struct Definition {
    pos: String,
    meaning: Vec<Meaning>,
}

#[derive(Debug)]
pub struct Meaning {
    pub description: String,
    pub examples: Vec<String>,
}

impl WordData {
    pub fn new(word: String, pos: String, meaning: Vec<Meaning>) -> Self {
        WordData {
            word,
            definitions: vec![Definition { pos, meaning }],
            pronunciation_url: None,
        }
    }
}
