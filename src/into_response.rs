use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Data not found")]
    NotFound,

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            },
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", inner);
        AppError::InternalServerError
    }
}