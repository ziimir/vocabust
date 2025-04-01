use axum::{
    extract::{State, Request},
    response::Response,
    middleware::Next,
    http::StatusCode,
    body::{to_bytes, Body}
};
use minijinja::context;

use super::super::AppState;

pub async fn error_middleware(
    state: State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let response = next.run(req).await;

    if response.status().is_client_error() || response.status().is_server_error() {
        let (header, body) = response.into_parts();

        let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);

        let html = match header.status {
            StatusCode::BAD_REQUEST => {
                let template = state.template_env.get_template("error_pages/400.html").unwrap();
                template.render(context! { message => body_str }).unwrap()
            },
            StatusCode::NOT_FOUND => {
                let template = state.template_env.get_template("error_pages/404.html").unwrap();
                template.render(context! { message => body_str }).unwrap()
            },
            _ => {
                let template = state.template_env.get_template("error_pages/500.html").unwrap();
                template.render(context! { message => body_str }).unwrap()
            },
        };

        return Response::builder()
            .status(header.status)
            .body(
                Body::from(html)
            )
            .unwrap();
    }

    response
}
