use axum::{Router, routing::post};

use crate::{
    app::AppState,
    modules::auth::controller::{login, register},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
