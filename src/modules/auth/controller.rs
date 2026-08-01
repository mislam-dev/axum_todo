use crate::modules::auth::jwt::{JwtPaylaod, create_jwt};
use crate::modules::user::dto::UserCreateDto;
use crate::modules::user::entities::users::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity,
};
use crate::modules::user::password::{hash_password, verify_passwrod};
use crate::{
    app::AppState,
    modules::auth::dto::{LoginResponse, LoginUserDto},
};
use axum::Json;
use axum::{Extension, http::StatusCode};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(payload): Json<LoginUserDto>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let find_user = UserEntity::find()
        .filter(UserColumn::Email.eq(payload.email))
        .one(&state.db)
        .await
        .unwrap();
    if find_user.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let find_user = find_user.unwrap();

    let verify = verify_passwrod(&find_user.password, &payload.password).await;

    if !verify {
        return Err(StatusCode::BAD_REQUEST);
    }

    let access_token = create_jwt(JwtPaylaod {
        email: find_user.email,
        sub: find_user.id.to_string(),
    })
    .unwrap();

    Ok(Json(LoginResponse {
        acess_token: access_token,
        refresh_token: "_".to_owned(),
    }))
}

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UserCreateDto>,
) -> Result<(), StatusCode> {
    let find_user = UserEntity::find()
        .filter(UserColumn::Email.eq(&payload.email))
        .one(&state.db)
        .await
        .unwrap();

    if !find_user.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let hash_password = hash_password(&payload.password).await;
    let active_user = UserActiveModel {
        email: Set(payload.email),
        password: Set(hash_password),
        ..Default::default()
    };

    let _ = active_user.insert(&state.db).await.unwrap();

    Ok(())
}
