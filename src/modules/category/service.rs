use super::dto::{CategoryCreateDto, CategoryItemResponse, CategoryUpdateDto};
use super::repository::CategoryRepository;
use crate::core::errors::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct CategoryService;

impl CategoryService {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<CategoryItemResponse>, AppError> {
        let categories = CategoryRepository::find(db).await?;
        let categories = categories
            .into_iter()
            .map(|c| CategoryItemResponse {
                id: c.id,
                name: c.name,
            })
            .collect::<Vec<CategoryItemResponse>>();
        Ok(categories)
    }
    pub async fn find_one(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<CategoryItemResponse, AppError> {
        let category = CategoryRepository::find_one(db, id).await?;

        let category = category.ok_or(AppError::NotFound("Category not found".to_string()))?;

        Ok(CategoryItemResponse {
            id: category.id,
            name: category.name,
        })
    }
    pub async fn create(
        db: &DatabaseConnection,
        dto: CategoryCreateDto,
    ) -> Result<CategoryItemResponse, AppError> {
        let category = CategoryRepository::create(db, dto).await?;

        Ok(CategoryItemResponse {
            id: category.id,
            name: category.name,
        })
    }
    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        dto: CategoryUpdateDto,
    ) -> Result<CategoryItemResponse, AppError> {
        let user = CategoryRepository::update(db, id, dto).await?;

        Ok(CategoryItemResponse {
            id: user.id,
            name: user.name,
        })
    }
    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        let _ = CategoryRepository::remove(db, id).await?;

        Ok(())
    }
}
