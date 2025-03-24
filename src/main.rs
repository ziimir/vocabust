use std::sync::Arc;
use minijinja::{path_loader, Environment};

mod server;

use server::{AppState, create_routes};

#[tokio::main]
async fn main() {
    let template_env = Environment::new();
    //template_env.set_loader(path_loader("templates"));
    //template_env.add_template("hello", "Hello {{ name }}!").unwrap();

    let app_state = AppState{ template_env };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, create_routes(app_state)).await.unwrap();
}
