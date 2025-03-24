use super::AppState;
use axum::{extract::State, routing::get, Router};
use minijinja::context;

const TEMPLATE_NAME: &str = "index";

async fn handler(State(state): State<AppState>) -> String {
    let template = state.template_env.get_template(TEMPLATE_NAME).unwrap();
    template.render(context!(name => "Dan")).unwrap().to_string()
}

pub fn create_index_router(state: &mut AppState) -> Router<AppState> {
    state.template_env.add_template(TEMPLATE_NAME,include_str!("./templates/index.html")).unwrap();

    Router::new().route("/", get(handler))
}
