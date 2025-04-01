use std::collections::HashMap;
use axum::{
    extract::{State, Query},
    response::Html,
};
use minijinja::context;

use crate::domain::{OxfordDictProvider, DictProviderError, translate_word_data};
use crate::infrastructure::{AppState, AppError};

pub async fn search_route_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    let template = state.template_env.get_template("search.html").unwrap();
    let query = query.get("q").map(|x| x.as_str()).unwrap_or("");

    let client = reqwest::Client::new();
    let oxford_provider = OxfordDictProvider::new(client);

    let (word_data, original_query_link) = oxford_provider.search_word(&query)
        .await
        .map_err(|err| {
            match err {
                DictProviderError::BadRequest(x) => AppError::BadRequest(x),
                DictProviderError::NotFound(x) => AppError::NotFound(x),
                DictProviderError::UrlParseError(x)
                | DictProviderError::ParseError(x)
                | DictProviderError::SelectError(x) => AppError::UnexpectedError(Some(x)),
                _ => AppError::UnexpectedError(None)
            }
        })?;

    let translations = translate_word_data(&word_data).await;

    Ok(Html(
        template.render(
            context!{
                query,
                original_query_link,
                data => word_data,
                translations => translations.definitions
        })
            .unwrap()
            .to_string()
    ))
}
