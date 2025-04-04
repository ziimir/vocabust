use scraper::{ElementRef, Html};

use super::super::super::{Meaning, WordData};

use super::DictProviderError;

pub trait DictProvider {
    fn definition(&self, data: &Html, source: String) -> Result<WordData, DictProviderError>;

    fn word(&self, content: &ElementRef) -> Option<String>;

    fn pos(&self, content: &ElementRef) -> Option<String>;

    fn meaning_list(&self, content: &ElementRef) -> Option<Vec<Meaning>>;

    fn pronunciation(&self, content: &ElementRef) -> Option<String>;
}
