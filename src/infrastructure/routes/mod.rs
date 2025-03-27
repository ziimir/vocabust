mod handlers;

use axum::{routing::get, Router};

use crate::infrastructure::{AppRoute, AppState};
use handlers::{home_route_handler, search_route_handler};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route(AppRoute::Home.pattern(), get(home_route_handler))
        .route(AppRoute::Search.pattern(), get(search_route_handler))
        .with_state(state)
}
