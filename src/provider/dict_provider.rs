use std::error::Error;

use reqwest::Client;
use scraper::Html;

pub trait DictProvider {
    fn new(client: Client) -> Self;

    async fn content(&self, query: &str) -> Result<Html, Box<dyn Error>>;
}
