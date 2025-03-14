use reqwest::Client;
use scraper::{Html, Selector};

use crate::provider::dict_provider::{DictProvider, DictProviderError};

pub struct OxfordDictProvider {
    client: Client
}

impl OxfordDictProvider {
    const CSS_SELECTOR: &str = "#entryContent";
}

impl DictProvider for OxfordDictProvider {
    const URL_TEMPLATE: &str = "https://www.oxfordlearnersdictionaries.com/search/english/?q={}";

    fn new(client: Client) -> Self {
        OxfordDictProvider { client }
    }

    async fn content(&self, query: &str) -> Result<Html, DictProviderError> {
        let url = Self::URL_TEMPLATE.replace("{}", query);
        let response = self.client.get(url).send().await?;

        let body = response.text().await?;

        let page = Html::parse_document(&body);
        let content_selector = Selector::parse(Self::CSS_SELECTOR)?;

        let content = page
            .select(&content_selector)
            .next()
            .ok_or_else(
                || DictProviderError::ParseError(
                    String::from(
                        format!("Element with selector {} not found", Self::CSS_SELECTOR)
                    )
                )
            )?;

        let fragment = Html::parse_fragment(&content.html());
        Ok(fragment)
    }
}
