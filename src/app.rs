use axum::{Router, routing::get};

use crate::modules::docs::docs_router;

pub fn app() -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(docs_router());

    router
}
