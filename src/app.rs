use crate::database::connect_db;
use crate::modules::auth::router::auth_router;
use crate::modules::category::router::category_router;
use crate::modules::docs::docs_router;
use crate::modules::todo::router::todo_router;
use crate::modules::user::router::user_router;
use axum::http::{Method, StatusCode};
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use std::env;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn app() -> Result<Router, Box<dyn std::error::Error>> {
    let db_url =
        env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set in the environment")?;

    let db = connect_db(&db_url)
        .await
        .map_err(|e| format!("Fatal: Colud not connect to database: {}", e))?;

    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/auth", auth_router())
        .nest("/api/users", user_router())
        .nest("/api/todos", todo_router())
        .nest("/api/categories", category_router())
        .merge(docs_router())
        .layer(cors)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .with_state(state);

    Ok(router)
}
