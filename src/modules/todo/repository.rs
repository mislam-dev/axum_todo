use crate::core::errors::error::AppError;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DeleteResult, EntityTrait};

use super::dto::{TodoCreateDto, TodoUpdateDto};
use super::entities::todos::{
    ActiveModel as TodosActiveModel, Column as TodosColumn, Entity as TodosEntity,
    Model as TodosModel,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, QueryFilter};
use uuid::Uuid;

pub struct TodosRepository;

impl TodosRepository {
    pub async fn find(
        db: &DatabaseConnection,
        filter: Option<Condition>,
    ) -> Result<Vec<TodosModel>, AppError> {
        let mut query = TodosEntity::find();
        if let Some(condition) = filter {
            query = query.filter(condition)
        }
        query.all(db).await.map_err(AppError::Database)
    }

    pub async fn find_one(
        db: &DatabaseConnection,
        id: i32,
        filter: Option<Condition>,
    ) -> Result<Option<TodosModel>, AppError> {
        let mut query = TodosEntity::find_by_id(id);

        if let Some(condition) = filter {
            query = query.filter(condition)
        }

        query.one(db).await.map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: TodoCreateDto,
    ) -> Result<TodosModel, AppError> {
        let new_todo = TodosActiveModel {
            title: Set(dto.title),
            completed: match dto.completed {
                Some(completed) => Set(completed),
                None => Set(false),
            },
            user_id: Set(user_id),
            ..Default::default()
        };
        new_todo.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        dto: TodoUpdateDto,
        filter: Option<Condition>,
    ) -> Result<TodosModel, AppError> {
        let existing_todo = Self::find_one(db, id, filter)
            .await?
            .ok_or(AppError::NotFound("User not found!".to_string()))?;

        let mut active_todo: TodosActiveModel = existing_todo.into();

        if let Some(title) = dto.title {
            active_todo.title = Set(title);
        }

        if let Some(completed) = dto.completed {
            active_todo.completed = Set(completed);
        }

        active_todo.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(
        db: &DatabaseConnection,
        id: i32,
        user_id: Uuid,
    ) -> Result<DeleteResult, AppError> {
        TodosEntity::delete_by_id(id)
            .filter(TodosColumn::UserId.eq(user_id))
            .exec(db)
            .await
            .map_err(AppError::Database)
    }
}
