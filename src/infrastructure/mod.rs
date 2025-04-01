use axum::{middleware::from_fn_with_state, Router};

mod app_error;
mod app_route;
mod app_state;
mod middleware;
mod page_routes;

pub use app_error::AppError;
pub use app_route::{build_url, AppRoute};
pub use app_state::AppState;

use page_routes::create_page_routes;

pub fn create_routes(state: AppState) -> Router {
    Router::new().merge(
        create_page_routes(state.clone())
            .layer(from_fn_with_state(state, middleware::error_middleware)),
    )
}
