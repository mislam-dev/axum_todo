use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash, verify};
pub async fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST)
        .map_err(|err| {
            eprintln!("Failed to hash password: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .unwrap()
}
pub async fn verify_passwrod(hash: &str, password: &str) -> bool {
    verify(password, hash)
        .map_err(|err| {
            eprintln!("Failed to verify password: {}", err);
            false
        })
        .unwrap_or(false)
}
