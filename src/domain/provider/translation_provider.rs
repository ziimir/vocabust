use futures::future::join_all;
use libretranslate::{translate_url, Language, Translation, TranslateError};

use crate::domain::WordData;

async fn translate(target: &str) -> Result<Translation, TranslateError> {
    translate_url(
        Language::English,
        Language::Russian,
        target,
        "http://192.168.0.254:5000",
        None,
    ).await
}

pub async fn translate_word_data(data: &WordData) -> Vec<String> {
    join_all(
        data.definitions
            .iter()
            .flat_map(|definition| &definition.meaning)
            .map(|meaning| translate(meaning.description.as_str()))
    )
        .await
        .iter()
        .map(|result| result.as_ref()
            .and_then(|tr| Ok(tr.output.clone()))
            .unwrap_or(String::from("Не удалось получить перевод"))
        )
        .collect::<Vec<String>>()
}
