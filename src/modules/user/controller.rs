use crate::{
    app::AppState,
    core::errors::error::AppError,
    modules::user::{
        dto::{IdParam, UserCreateDto, UserItemResponse, UserUpdateDto},
        service::UserService,
    },
};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<UserItemResponse>>, AppError> {
    let users = UserService::find(&state.db).await?;
    Ok(Json(users))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::find_one(&state.db, id.0).await?;
    Ok(Json(user))
}

pub async fn add(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateDto>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::create(&state.db, payload).await?;
    Ok(Json(user))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<UserUpdateDto>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::update(&state.db, id.0, payload).await?;
    Ok(Json(user))
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), AppError> {
    let _ = UserService::remove(&state.db, id.0).await?;

    Ok(())
}
