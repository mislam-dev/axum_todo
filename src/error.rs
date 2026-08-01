use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Authentication Error: {0}")]
    Unauthorized(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (statuus_code, msg) = match self {
            AppError::Database(err) => {
                eprintln!("Datbase error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
            AppError::Uuid(err) => {
                eprintln!("Invalid UUID error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid ID format: {}", err),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({
          "error": msg,
        }));
        (statuus_code, body).into_response()
    }
}
