use super::entities::category::{ActiveModel as CategoryActiveModel, Entity as CategoryEntity};

use crate::{
    app::AppState,
    modules::category::dto::{CategoryCreateDto, CategoryItemResponse, CategoryUpdateDto, IdParam},
};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub async fn list(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<CategoryItemResponse>>, StatusCode> {
    let categories = CategoryEntity::find().all(&state.db).await.unwrap();
    let categories_data = categories
        .into_iter()
        .map(|c| CategoryItemResponse {
            id: c.id,
            name: c.name,
        })
        .collect::<Vec<CategoryItemResponse>>();
    Ok(Json(categories_data))
}

pub async fn show(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<CategoryItemResponse>, StatusCode> {
    let category = CategoryEntity::find_by_id(id.0)
        .one(&state.db)
        .await
        .unwrap();

    if category.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let category = category.unwrap();

    Ok(Json(CategoryItemResponse {
        id: category.id,
        name: category.name,
    }))
}

pub async fn add(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CategoryCreateDto>,
) -> Result<Json<CategoryItemResponse>, StatusCode> {
    let new_category = CategoryActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let inserted_category = new_category.insert(&state.db).await.unwrap();

    Ok(Json(CategoryItemResponse {
        id: inserted_category.id,
        name: inserted_category.name,
    }))
}

pub async fn update(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<CategoryUpdateDto>,
) -> Result<Json<CategoryItemResponse>, StatusCode> {
    let existing_category = CategoryEntity::find_by_id(id.0)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Best practice instead of .unwrap
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_category: CategoryActiveModel = existing_category.into();

    if let Some(name) = payload.name {
        active_category.name = Set(name);
    }

    let updated_category = active_category
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CategoryItemResponse {
        id: updated_category.id,
        name: updated_category.name,
    }))
}

pub async fn remove(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), StatusCode> {
    let delete_result = CategoryEntity::delete_by_id(id.0)
        .exec(&state.db)
        .await
        .unwrap();

    if delete_result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}
