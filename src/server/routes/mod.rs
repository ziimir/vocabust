mod index;

use axum::Router;

use self::index::create_index_router;
use super::AppState;

pub fn create_routes(mut state: AppState) -> Router {
    Router::new()
        .merge(create_index_router(&mut state))
        .with_state(state)
}
