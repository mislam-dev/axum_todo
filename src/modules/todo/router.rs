use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

use crate::{
    app::AppState,
    modules::{
        auth::jwt::Claims,
        todo::controller::{add, list, remove, show, update},
    },
};

pub fn todo_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{id}", get(show))
        .route("/", post(add))
        .route("/{id}", patch(update))
        .route("/{id}", delete(remove))
        .route_layer(middleware::from_extractor::<Claims>())
}
