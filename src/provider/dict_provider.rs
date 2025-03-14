use thiserror::Error;

use reqwest::Client;
use scraper::Html;
use scraper::error::SelectorErrorKind;

#[derive(Error, Debug)]
pub enum DictProviderError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for DictProviderError {
    fn from(err: SelectorErrorKind<'a>) -> Self {
        DictProviderError::ParseError(format!("{:?}", err))
    }
}

pub trait DictProvider {
    const URL_TEMPLATE: &str;

    fn new(client: Client) -> Self;

    async fn content(&self, query: &str) -> Result<Html, DictProviderError>;
}
