use crate::{
    core::errors::error::AppError,
    modules::user::dto::{UserCreateDto, UserUpdateDto},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DeleteResult, EntityTrait};

use super::entities::users::{
    ActiveModel as UsersActiveModel, Entity as UsersEntity, Model as UserModel,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<UserModel>, AppError> {
        UsersEntity::find()
            .all(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_one(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<UserModel>, AppError> {
        UsersEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_data: UserCreateDto,
    ) -> Result<UserModel, AppError> {
        let new_user = UsersActiveModel {
            name: Set(user_data.name),
            email: Set(user_data.email),
            password: Set(user_data.password),
            ..Default::default()
        };
        new_user.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        user_data: UserUpdateDto,
    ) -> Result<UserModel, AppError> {
        let existing_user = Self::find_one(db, id)
            .await?
            .ok_or(AppError::NotFound("User not found!".to_string()))?;

        let mut active_user: UsersActiveModel = existing_user.into();

        if let Some(name) = user_data.name {
            active_user.name = Set(name);
        }

        active_user.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
        UsersEntity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)
    }
}
