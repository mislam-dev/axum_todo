use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Authentication Error: {0}")]
    Unauthorized(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal Server Error: {0}")]
    #[allow(dead_code)]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(err) => {
                eprintln!("Datbase error: {:?}", err);
                let body = Json(json!({
                  "error": "An internal server erorr occured".to_string()
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::Uuid(err) => {
                eprintln!("Invalid UUID error: {:?}", err);

                let body = Json(json!({
                  "error":format!("Invalid ID format: {}", err),
                }));

                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::Validation(errors) => {
                let mut err_map = serde_json::Map::new();

                for (field, field_errors) in errors.field_errors() {
                    let messages: Vec<String> = field_errors
                        .iter()
                        .map(|e| {
                            // Use custom message if provided, otherwise default to the rule code (e.g., "email")
                            e.message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| e.code.to_string())
                        })
                        .collect();

                    err_map.insert(field.to_string(), json!(messages));
                }

                let body = Json(json!({
                    "error": "Validation failed",
                    "details": err_map // e.g., { "email": ["Invalid email format"], "password": ["Too short"] }
                }));

                (StatusCode::BAD_REQUEST, body).into_response()
            }

            AppError::Unauthorized(msg) => {
                let body = Json(json!({
                  "error": msg,
                }));

                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            AppError::NotFound(msg) => {
                let body = Json(json!({
                  "error": msg,
                }));

                (StatusCode::NOT_FOUND, body).into_response()
            }
            AppError::BadRequest(msg) => {
                let body = Json(json!({
                  "error": msg,
                }));

                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::InternalServerError(msg) => {
                let body = Json(json!({
                  "error": msg,
                }));

                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}
