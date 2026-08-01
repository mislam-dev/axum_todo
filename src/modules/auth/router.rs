use axum::{Router, routing::post};

use crate::modules::auth::controller::{login, register};

pub fn auth_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
