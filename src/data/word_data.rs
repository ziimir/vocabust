struct WordData {
    word: String,
    definitions: Vec<Definition>,
    pronunciation_url: Optional<String>,
}

struct Definition {
    pos: String,
    meaning: String,
    example: Vec<String>,
}
