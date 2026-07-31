use std::env;

use axum::Extension;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::database::database::connect_db;
use crate::modules::docs::docs_router;
use crate::modules::todo::router::todo_router;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn app() -> Router {
    let db_url = env::var("DATABASE_URL").expect("Database url must set");

    let db = connect_db(&db_url).await;

    let state = AppState { db };

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/todos", todo_router())
        .layer(Extension(state))
        .merge(docs_router());

    router
}
