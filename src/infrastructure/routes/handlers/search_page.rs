use std::collections::HashMap;
use axum::{
    extract::{State, Query},
    response::{Html, IntoResponse},
};
use minijinja::context;

use crate::domain::{OxfordDictProvider, translate_word_data};
use crate::infrastructure::AppState;

pub async fn search_route_handler(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let template = state.template_env.get_template("search.html").unwrap();
    let query = query.get("q").map(|x| x.as_str()).unwrap_or("");

    let client = reqwest::Client::new();
    let oxford_provider = OxfordDictProvider::new(client);

    let (word_data, original_query_link) = oxford_provider.search_word(&query).await.unwrap();

    let translations = translate_word_data(&word_data).await;

    println!("{:#?}", translations);

    Html(template.render(context!{ query, original_query_link, data => word_data }).unwrap().to_string())
}
