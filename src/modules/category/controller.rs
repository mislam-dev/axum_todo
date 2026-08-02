use crate::core::validation::validation::JsonValidate;
use crate::{app::AppState, core::errors::error::AppError};

use super::dto::{CategoryCreateDto, CategoryItemResponse, CategoryUpdateDto, IdParam};
use super::service::CategoryService;
use axum::extract::State;
use axum::{Json, extract::Path};

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryItemResponse>>, AppError> {
    let response = CategoryService::find(&state.db).await?;
    Ok(Json(response))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let category = CategoryService::find_one(&state.db, id.0).await?;
    Ok(Json(category))
}

pub async fn add(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<CategoryCreateDto>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let new_category = CategoryService::create(&state.db, payload).await?;
    Ok(Json(new_category))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
    JsonValidate(payload): JsonValidate<CategoryUpdateDto>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let updated_category = CategoryService::update(&state.db, id.0, payload).await?;

    Ok(Json(updated_category))
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), AppError> {
    let _ = CategoryService::remove(&state.db, id.0).await?;

    Ok(())
}
