use super::entities::users::Column as UserColumn;
use super::{
    dto::{UserCreateDto, UserItemResponse, UserItemWithPassword, UserUpdateDto},
    repository::UserRepository,
};
use crate::core::errors::error::AppError;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};

use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<UserItemResponse>, AppError> {
        let users = UserRepository::find(db).await?;
        let users_data = users
            .into_iter()
            .map(|c| UserItemResponse {
                id: c.id,
                name: c.name,
                email: c.email,
            })
            .collect::<Vec<UserItemResponse>>();
        Ok(users_data)
    }
    pub async fn find_one(db: &DatabaseConnection, id: Uuid) -> Result<UserItemResponse, AppError> {
        let user = UserRepository::find_by_id(db, id).await?;

        let user = user.ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
    pub async fn create(
        db: &DatabaseConnection,
        dto: UserCreateDto,
    ) -> Result<UserItemResponse, AppError> {
        let user = UserRepository::create(db, dto).await?;

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        dto: UserUpdateDto,
    ) -> Result<UserItemResponse, AppError> {
        let user = UserRepository::update(db, id, dto).await?;

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        let _ = UserRepository::remove(db, id).await?;

        Ok(())
    }

    pub async fn find_by_email_with_password(
        db: &DatabaseConnection,
        email: &String,
    ) -> Result<UserItemWithPassword, AppError> {
        let filter = Condition::all().add(UserColumn::Email.eq(email));
        let user = UserRepository::find_one(db, filter).await?;
        let user = user.ok_or(AppError::NotFound("User not found".to_string()))?;
        Ok(UserItemWithPassword {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
        })
    }
}
