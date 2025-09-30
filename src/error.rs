use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found.").into_response(),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            AppError::Internal(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

impl<E: std::error::Error> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError::Internal(err.to_string())
    }
}
