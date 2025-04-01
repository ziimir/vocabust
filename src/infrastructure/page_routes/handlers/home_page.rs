use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use minijinja::context;

use crate::infrastructure::{AppState, AppRoute, build_url};

pub async fn home_route_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = state.template_env.get_template("home.html").unwrap();

    Html(template.render(context!{search_url => build_url(AppRoute::Search)}).unwrap().to_string())
}
