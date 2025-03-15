use reqwest::Client;
use scraper::{Html, Selector, ElementRef};

use crate::data::{WordData, Meaning};

use super::dict_provider::{DictProvider, DictProviderError};

pub struct OxfordDictProvider {
    client: Client
}

impl DictProvider for OxfordDictProvider {
    const URL_TEMPLATE: &str = "https://www.oxfordlearnersdictionaries.com/search/english/?q={}";

    fn new(client: Client) -> Self {
        OxfordDictProvider { client }
    }

    async fn word_definition(&self, query: &str) -> Result<WordData, DictProviderError> {
        let url = Self::URL_TEMPLATE.replace("{}", query);

        let response = self.client.get(url).send().await?;
        let body = response.text().await?;

        let page = Html::parse_document(&body);

        let content = page
            .select(&Selector::parse("#entryContent").unwrap())
            .next()
            .ok_or(DictProviderError::SelectError(
                String::from(format!("Element with selector #entryContent not found"))
            ))?;

        let word = self
            .word(&content)
            .ok_or(DictProviderError::SelectError(String::from("word is not found")))?;

        let pos = self
            .pos(&content)
            .ok_or(DictProviderError::SelectError(String::from("pos is not found")))?;

        let meaning_list = self
            .meaning_list(&content)
            .ok_or(DictProviderError::SelectError(String::from("meanings is not found")))?;

        let pronunciation = self
            .pronunciation(&content);

        Ok(WordData::new(word, pos, meaning_list, pronunciation))
    }
}

impl OxfordDictProvider {
    fn word<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let word = content
            .select(&Selector::parse(".headword").unwrap())
            .next()?
            .text()
            .collect::<String>();

        if word.is_empty() {
            return None;
        }

        Some(word)
    }

    fn pos<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let pos = content
            .select(&Selector::parse(".headword + .pos").unwrap())
            .next()?
            .text()
            .collect::<String>();

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
                    .collect::<String>();

                let examples = sense_item_element
                    .select(&Selector::parse(".examples li").unwrap())
                    .map(|example_item_element| {
                        example_item_element
                            .text()
                            .collect::<String>()
                        })
                    .collect();

                if def.is_empty() {
                    return None;
                }

                Some(Meaning { description: def, examples })
            })
            .flatten()
            .collect();

        if meanings.len() == 0 {
            return None;
        }

        Some(meanings)
    }

    fn pronunciation<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let pronunciation = content
            .select(&Selector::parse(".phonetics .phons_n_am .sound").unwrap())
            .next()?
            .attr("data-src-mp3")
            .map(|x| String::from(x));

        pronunciation
    }
}
