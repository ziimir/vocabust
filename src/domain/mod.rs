mod data;
mod provider;

pub use data::{Meaning, TranslatedWordData, WordData};
pub use provider::{translate_word_data, OxfordDictProvider};
