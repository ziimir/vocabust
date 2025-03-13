mod provider;

use provider::{DictProvider, OxfordDictProvider};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let query = "hide";

    let oxford_provider = OxfordDictProvider::new(client);
    let content = oxford_provider.content(&query)
        .await
        .unwrap_or_else(|err| panic!("Error during fetch and parse: {}", err));

    println!("{}", content.html());
}

