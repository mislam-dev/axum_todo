use super::entities::users::{ActiveModel as UsersActiveModel, Entity as UsersEntity};

use crate::{
    app::AppState,
    modules::user::dto::{IdParam, UserCreateDto, UserItemResponse, UserUpdateDto},
};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub async fn list(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<UserItemResponse>>, StatusCode> {
    let users = UsersEntity::find().all(&state.db).await.unwrap();
    let users_data = users
        .into_iter()
        .map(|c| UserItemResponse {
            id: c.id,
            name: c.name,
            email: c.email,
        })
        .collect::<Vec<UserItemResponse>>();
    Ok(Json(users_data))
}

pub async fn show(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<UserItemResponse>, StatusCode> {
    let user = UsersEntity::find_by_id(id.0).one(&state.db).await.unwrap();

    if user.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let user = user.unwrap();

    Ok(Json(UserItemResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
}

pub async fn add(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UserCreateDto>,
) -> Result<Json<UserItemResponse>, StatusCode> {
    let new_user = UsersActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let inserted_user = new_user.insert(&state.db).await.unwrap();

    Ok(Json(UserItemResponse {
        id: inserted_user.id,
        name: inserted_user.name,
        email: inserted_user.email,
    }))
}

pub async fn update(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<UserUpdateDto>,
) -> Result<Json<UserItemResponse>, StatusCode> {
    let existing_user = UsersEntity::find_by_id(id.0)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Best practice instead of .unwrap
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_user: UsersActiveModel = existing_user.into();

    if let Some(name) = payload.name {
        active_user.name = Set(name);
    }

    let updated_user = active_user
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserItemResponse {
        id: updated_user.id,
        name: updated_user.name,
        email: updated_user.email,
    }))
}

pub async fn remove(
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), StatusCode> {
    let delete_result = UsersEntity::delete_by_id(id.0)
        .exec(&state.db)
        .await
        .unwrap();

    if delete_result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}
