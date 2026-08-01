use crate::app::AppState;
use crate::core::errors::error::AppError;
use crate::modules::auth::jwt::Claims;
use crate::modules::todo::dto::{IdParam, TodoCreateDto, TodoItemResponse, TodoUpdateDto};
use crate::modules::todo::service::TodosService;
use axum::{Extension, Json, extract::Path};

pub async fn list(
    claims: Claims,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<TodoItemResponse>>, AppError> {
    let response = TodosService::find(&state.db, claims.sub).await?;
    Ok(Json(response))
}

pub async fn show(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<TodoItemResponse>, AppError> {
    let response = TodosService::find_one(&state.db, claims.sub, id.0).await?;
    Ok(Json(response))
}

pub async fn add(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Json(payload): Json<TodoCreateDto>,
) -> Result<Json<TodoItemResponse>, AppError> {
    let todo = TodosService::create(&state.db, claims.sub, payload).await?;
    Ok(Json(todo))
}

pub async fn update(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<TodoUpdateDto>,
) -> Result<Json<TodoItemResponse>, AppError> {
    let update_todo = TodosService::update(&state.db, id.0, claims.sub, payload).await?;

    Ok(Json(update_todo))
}

pub async fn remove(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), AppError> {
    TodosService::remove(&state.db, id.0, claims.sub).await?;
    Ok(())
}
