use super::AppState;
use axum::{extract::State, routing::get, Router};
use minijinja::context;

async fn handler(State(state): State<AppState>) -> String {
    //let template = state.template_env.get_template("hello").unwrap();
    let template = state.template_env.get_template("index").unwrap();
    template.render(context!(name => "Dan")).unwrap().to_string()
}

pub fn create_index_router(state: &mut AppState) -> Router<AppState> {
    //state.template_env.add_template("hello", "Hello {{ name }}!").unwrap();
    state.template_env.add_template("index",include_str!("./index.html")).unwrap();

    Router::new().route("/", get(handler))
}
