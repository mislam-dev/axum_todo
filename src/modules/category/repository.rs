use crate::core::errors::AppError;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DeleteResult, EntityTrait};

use super::dto::{CategoryCreateDto, CategoryUpdateDto};
use super::entities::category::{
    ActiveModel as CategoryActiveModel, Entity as CategoryEntity, Model as CategoryModel,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<CategoryModel>, AppError> {
        CategoryEntity::find()
            .all(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_one(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<CategoryModel>, AppError> {
        CategoryEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_data: CategoryCreateDto,
    ) -> Result<CategoryModel, AppError> {
        let new_category = CategoryActiveModel {
            name: Set(user_data.name),
            ..Default::default()
        };
        new_category.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        user_data: CategoryUpdateDto,
    ) -> Result<CategoryModel, AppError> {
        let existing_category = Self::find_one(db, id)
            .await?
            .ok_or(AppError::NotFound("Category not found!".to_string()))?;

        let mut active_category: CategoryActiveModel = existing_category.into();

        if let Some(name) = user_data.name {
            active_category.name = Set(name);
        }

        active_category.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
        CategoryEntity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)
    }
}
