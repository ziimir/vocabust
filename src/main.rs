use std::fs::File;

mod data;
mod provider;
mod exporter;

use provider::OxfordDictProvider;
use exporter::create_anki_file;

#[tokio::main]
async fn main() {
    let query = "elaborate";

    let client = reqwest::Client::new();
    let oxford_provider = OxfordDictProvider::new(client);

    let word_data = oxford_provider.search_word(&query).await.unwrap();

    let mut file = File::create("anki.txt").unwrap();

    create_anki_file(&mut file, &word_data).unwrap();

    println!("done");
}

