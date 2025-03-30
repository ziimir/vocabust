use futures::future::join_all;
use libretranslate::{translate_url, Language, Translation, TranslateError};

use crate::domain::{WordData, TranslatedWordData};

async fn translate(target: &str) -> Result<Translation, TranslateError> {
    translate_url(
        Language::English,
        Language::Russian,
        target,
        "http://192.168.0.254:5000",
        None,
    ).await
}

pub async fn translate_word_data(data: &WordData) -> TranslatedWordData {
    let translate_definitions = join_all(
        data
            .definitions
            .iter()
            .map(|definition| async {
                join_all(definition.meaning
                    .iter()
                    .map(|meaning| async {
                        translate(meaning.description.as_str())
                            .await
                            .and_then(|tr| Ok(tr.output))
                            .unwrap_or(String::from("Не удалось получить перевод"))
                    })
                )
                    .await
            })
    )
        .await;

    TranslatedWordData::new(translate_definitions)
}
