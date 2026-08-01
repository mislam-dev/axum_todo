use crate::{app::AppState, core::errors::error::AppError};

use super::dto::{CategoryCreateDto, CategoryItemResponse, CategoryUpdateDto, IdParam};
use super::service::CategoryService;
use axum::{Extension, Json, extract::Path};

pub async fn list(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<CategoryItemResponse>>, AppError> {
    let response = CategoryService::find(&state.db).await?;
    Ok(Json(response))
}

pub async fn show(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let category = CategoryService::find_one(&state.db, id.0).await?;
    Ok(Json(category))
}

pub async fn add(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CategoryCreateDto>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let new_category = CategoryService::create(&state.db, payload).await?;
    Ok(Json(new_category))
}

pub async fn update(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<CategoryUpdateDto>,
) -> Result<Json<CategoryItemResponse>, AppError> {
    let updated_category = CategoryService::update(&state.db, id.0, payload).await?;

    Ok(Json(updated_category))
}

pub async fn remove(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), AppError> {
    let _ = CategoryService::remove(&state.db, id.0).await?;

    Ok(())
}
