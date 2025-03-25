use scraper::error::SelectorErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DictProviderError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Element not found: {0}")]
    SelectError(String),

    #[error("Fail to parser url: {0}")]
    UrlParseError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for DictProviderError {
    fn from(err: SelectorErrorKind<'a>) -> Self {
        DictProviderError::ParseError(format!("{:?}", err))
    }
}
