use crate::core::errors::AppError;
use crate::core::validation::JsonValidate;
use crate::modules::auth::service::AuthService;
use crate::modules::user::dto::UserCreateDto;
use crate::{
    app::AppState,
    modules::auth::dto::{LoginResponse, LoginUserDto},
};
use axum::Json;
use axum::extract::State;

pub async fn login(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<LoginUserDto>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = AuthService::login(&state.db, payload).await?;

    Ok(Json(response))
}

pub async fn register(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<UserCreateDto>,
) -> Result<(), AppError> {
    let _ = AuthService::register(&state.db, payload).await?;

    Ok(())
}
