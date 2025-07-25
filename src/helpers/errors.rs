use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DbError(#[from] sqlx::Error),

    #[error("JSON serialization/deserialization error")]
    Serde(#[from] serde_json::Error),

    #[error("Internal server error")]
    Internal,

    #[error("Invalid input: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Password hashing failed")]
    HashingFailed(String),

    #[error("Password verification failed")]
    VerificationFailed,

    #[error("Email already exists")]
    Conflict,

    #[error("Resource not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::VerificationFailed => StatusCode::UNAUTHORIZED,
            AppError::HashingFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DbError(_) | AppError::Serde(_) | AppError::Internal => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, self.to_string()).into_response()
    }
}
