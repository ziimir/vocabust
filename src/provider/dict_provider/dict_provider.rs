use reqwest::Client;

use crate::data::WordData;

use super::DictProviderError;

pub trait DictProvider {
    const URL_TEMPLATE: &str;

    fn new(client: Client) -> Self;

    async fn word_definition(&self, query: &str) -> Result<WordData, DictProviderError>;
}
