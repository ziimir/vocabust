use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    UnexpectedError(Option<String>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, text) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::UnexpectedError(message) => (
                StatusCode::SERVICE_UNAVAILABLE,
                message.unwrap_or(String::from("Unexpected Error")),
            ),
        };

        (status, text).into_response()
    }
}
