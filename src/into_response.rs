use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorCode {
    NotFound,
    BadRequest,
    ValidationError,
    InternalServerError,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::BadRequest => "BAD_REQUEST",
            ErrorCode::ValidationError => "VALIDATION_ERROR",
            ErrorCode::InternalServerError => "INTERNAL_SERVER_ERROR",
        };
        write!(f, "{}", s)
    }
}

// 1. Standard struct for JSON error responses
#[derive(Serialize)]
pub struct ErrorResponse<T = ()> {
    pub success: bool,
    pub code: String,
    pub message: String,
    
    // Skip this field from JSON when value is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<T>,
}

// 2. AppError: Wrapper for various application errors
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error")]
    Validation(validator::ValidationErrors),

    #[error("Internal server error")]
    InternalServerError,
}

// Helper: Format ValidationErrors into a simple HashMap of field -> [error_message]
fn format_validation_errors(err: validator::ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    
    for (field, field_errors) in err.into_errors() {
        if let validator::ValidationErrorsKind::Field(validation_errors) = field_errors {
            let mut messages = Vec::new();
            for e in validation_errors {
                // Use the validation error message if present, otherwise fallback to the error code (e.g., "length")
                if let Some(msg) = e.message {
                    messages.push(msg.into_owned());
                } else {
                    messages.push(e.code.into_owned());
                }
            }
            errors.insert(field.to_string(), messages);
        }
    }
    
    errors
}

// 3. Mapping AppError -> Standard JSON Http Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Automatically extract the error message from the #[error("...")] macro definition
        let message = self.to_string();

        let (status, code, errors) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorCode::NotFound.to_string(),
                None,
            ),
            AppError::BadRequest(_) => (
                StatusCode::BAD_REQUEST,
                ErrorCode::BadRequest.to_string(),
                None,
            ),
            AppError::Validation(err) => (
                StatusCode::BAD_REQUEST,
                ErrorCode::ValidationError.to_string(),
                Some(format_validation_errors(err)),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::InternalServerError.to_string(),
                None,
            ),
        };

        // Dynamic JSON struct where type `T` depends on the `errors` type (HashMap or ())
        let body = Json(ErrorResponse {
            success: false,
            code,
            message,
            errors,
        });

        (status, body).into_response()
    }
}

// 4. Automatic conversion from SQLx Error to AppError
impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", inner);
        
        match inner {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::InternalServerError, // Do not leak DB error details to the client
        }
    }
}