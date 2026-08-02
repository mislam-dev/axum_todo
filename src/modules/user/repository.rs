use crate::{
    core::errors::error::AppError,
    modules::user::{
        dto::{UserCreateDto, UserUpdateDto},
        password::hash_password,
    },
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, Condition, DeleteResult, EntityTrait, QueryFilter,
};

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

    pub async fn find_by_id(
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
        dto: UserCreateDto,
    ) -> Result<UserModel, AppError> {
        let hash_p = hash_password(&dto.password).await?;
        let new_user = UsersActiveModel {
            name: Set(dto.name),
            email: Set(dto.email),
            password: Set(hash_p),
            ..Default::default()
        };
        new_user.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        user_data: UserUpdateDto,
    ) -> Result<UserModel, AppError> {
        let existing_user = Self::find_by_id(db, id)
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

    pub async fn find_one(
        db: &DatabaseConnection,
        filter: Condition,
    ) -> Result<Option<UserModel>, AppError> {
        UsersEntity::find()
            .filter(filter)
            .one(db)
            .await
            .map_err(AppError::Database)
    }
}
