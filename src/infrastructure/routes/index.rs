use axum::{extract::State, routing::get, Router};
use minijinja::context;

use crate::infrastructure::AppState;

async fn handler(State(state): State<AppState>) -> String {
    let template = state.template_env.get_template("index.html").unwrap();
    template.render(context!(name => "Dan")).unwrap().to_string()
}

pub fn create_index_router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}
