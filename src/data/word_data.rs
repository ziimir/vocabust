#[derive(Debug)]
pub struct WordData {
    pub word: String,
    pub definitions: Vec<Definition>,
    pub pronunciation_url: Option<String>,
}

#[derive(Debug)]
pub struct Definition {
    pub pos: String,
    pub meaning: Vec<Meaning>,
}

#[derive(Debug)]
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
    ) -> Self {
        WordData {
            word,
            definitions: vec![Definition { pos, meaning }],
            pronunciation_url,
        }
    }
}
