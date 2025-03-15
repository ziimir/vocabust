use reqwest::Client;
use scraper::{Html, Selector, ElementRef};

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

    async fn content(&self, query: &str) -> Result<String, DictProviderError> {
        let url = Self::URL_TEMPLATE.replace("{}", query);
        let response = self.client.get(url).send().await?;

        let body = response.text().await?;

        let page = Html::parse_document(&body);

        let content = page
            .select(&OxfordSelectors::content())
            .next()
            .ok_or_else(
                || DictProviderError::ParseError(
                    String::from(
                        format!("Element with selector {:?} not found", OxfordSelectors::content())
                    )
                )
            )?;

        let word = self.word(&content).await;

        Ok(word)
    }
}

impl OxfordDictProvider {
    pub async fn word<'a>(&self, content: &ElementRef<'a>) -> String {
        let word = content
            .select(&OxfordSelectors::word())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        String::from(word)
    }
}
