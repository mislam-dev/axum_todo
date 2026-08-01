use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::modules::user::controller::{add, list, remove, show, update};

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/{id}", get(show))
        .route("/", post(add))
        .route("/{id}", patch(update))
        .route("/{id}", delete(remove))
}
