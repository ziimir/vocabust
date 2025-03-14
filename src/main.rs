mod data;
mod provider;

use provider::{DictProvider, OxfordDictProvider};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let query = "hide";

    let oxford_provider = OxfordDictProvider::new(client);

    let word_data = oxford_provider.word_definition(&query)
        .await
        .unwrap_or_else(|err| panic!("Error during fetch and parse: {}", err));

    println!("{:#?}", word_data);
}

