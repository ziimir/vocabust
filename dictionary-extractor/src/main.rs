use std::sync::Arc;
use minijinja::{Environment, path_loader};

mod infrastructure;
mod domain;

use infrastructure::{AppState, create_routes};

#[tokio::main]
async fn main() {
    let mut template_env = Environment::new();
    template_env.set_loader(path_loader("templates"));

    let app_state = AppState{ template_env };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, create_routes(app_state)).await.unwrap();
}
