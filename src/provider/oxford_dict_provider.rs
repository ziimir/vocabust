use reqwest::Client;
use scraper::{Html, Selector, ElementRef};

use crate::data::{WordData, Meaning};

use super::dict_provider::{DictProvider, DictSelectors, DictProviderError};

struct OxfordSelectors {}

impl DictSelectors for OxfordSelectors {
    fn content() -> Selector {
        Selector::parse("#entryContent").unwrap()
    }

    fn word() -> Selector {
        Selector::parse(".headword").unwrap()
    }

    fn pos() -> Selector {
        Selector::parse(".headword + .pos").unwrap()
    }

    fn meaning_list() -> Selector {
        Selector::parse(".senses_multiple > .sense > .def").unwrap()
    }

    fn example_list() -> Selector {
        Selector::parse(".senses_multiple > .sense > .examples > li").unwrap()
    }

    fn poi_links() -> Option<Selector> {
        None
    }
}

pub struct OxfordDictProvider {
    client: Client
}

impl DictProvider for OxfordDictProvider {
    const URL_TEMPLATE: &str = "https://www.oxfordlearnersdictionaries.com/search/english/?q={}";

    fn new(client: Client) -> Self {
        OxfordDictProvider { client }
    }

    async fn content(&self, query: &str) -> Result<WordData, DictProviderError> {
        let url = Self::URL_TEMPLATE.replace("{}", query);
        let response = self.client.get(url).send().await?;

        let body = response.text().await?;

        let page = Html::parse_document(&body);

        let content = page
            .select(&OxfordSelectors::content())
            .next()
            .ok_or_else(
                || DictProviderError::SelectError(
                    String::from(
                        format!("Element with selector {:?} not found", OxfordSelectors::content())
                    )
                )
            )?;

        let word = self
            .word(&content)
            .ok_or(DictProviderError::SelectError(String::from("word is not found")))?;

        let pos = self
            .pos(&content)
            .ok_or(DictProviderError::SelectError(String::from("pos is not found")))?;

        let meaning_list = self
            .meaning_list(&content)
            .ok_or(DictProviderError::SelectError(String::from("meanings is not found")))?;

        Ok(WordData::new(word, pos, meaning_list))
    }
}

fn fold_strings_iter(acc: String, text: &str) -> String {
    acc + text
}

impl OxfordDictProvider {
    fn word<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let word = content
            .select(&OxfordSelectors::word())
            .next()?
            .text()
            .fold(String::from(""), fold_strings_iter);

        if word.is_empty() {
            return None;
        }

        Some(word)
    }

    fn pos<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let pos = content
            .select(&OxfordSelectors::pos())
            .next()?
            .text()
            .fold(String::from(""), fold_strings_iter);

        if pos.is_empty() {
            return None;
        }

        Some(pos)
    }

    fn meaning_list<'a>(&self, content: &ElementRef<'a>) -> Option<Vec<Meaning>> {
        let selector = Selector::parse(".senses_multiple > .sense").unwrap();

        let meanings: Vec<Meaning> = content
            .select(&selector)
            .map(|sense_item_element| {
                let def = sense_item_element
                    .select(&Selector::parse(".def").unwrap())
                    .next()?
                    .text()
                    .fold(String::from(""), fold_strings_iter);

                let examples = sense_item_element
                    .select(&Selector::parse(".examples li").unwrap())
                    .map(|example_item_element| {
                        example_item_element
                            .text()
                            .fold(String::from(""), fold_strings_iter)
                        })
                    .collect();

                if def.is_empty() {
                    return None;
                }

                Some(Meaning { description: def, examples })
            })
            .filter_map(|x| x)
            .collect();

        if meanings.len() == 0 {
            return None;
        }

        Some(meanings)
    }
}
