use reqwest::Client;
use scraper::Html;

use super::DictProviderError;

pub trait DictProvider {
    const URL_TEMPLATE: &str;

    fn new(client: Client) -> Self;

    async fn content(&self, query: &str) -> Result<(String, String, Vec<String>), DictProviderError>;
}
