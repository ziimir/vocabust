use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

use crate::provider::dict_provider::DictProvider;

const OXFORD_DICTIONARY_SEARCH_URL_TEMPLATE: &str =
    "https://www.oxfordlearnersdictionaries.com/search/english/?q={}";
const CONTENT_SELECTOR: &str = "#entryContent";

pub struct OxfordDictProvider {
    client: Client
}

impl DictProvider for OxfordDictProvider {
    fn new(client: Client) -> Self {
        OxfordDictProvider { client }
    }

    async fn content(&self, query: &str) -> Result<Html, Box<dyn Error>> {
        let url = OXFORD_DICTIONARY_SEARCH_URL_TEMPLATE.replace("{}", query);
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;

        let page = Html::parse_document(&body);
        let content_selector = Selector::parse(CONTENT_SELECTOR)?;

        let content = page
            .select(&content_selector)
            .next()
            .ok_or_else(|| format!("Element with selector {} not found", CONTENT_SELECTOR))?;

        let fragment = Html::parse_fragment(&content.html());
        Ok(fragment)
    }
}
