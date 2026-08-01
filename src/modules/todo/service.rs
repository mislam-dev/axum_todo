use super::dto::{TodoCreateDto, TodoUpdateDto};
use super::entities::todos::Column as TodosColumn;
use crate::core::errors::error::AppError;
use crate::modules::todo::dto::TodoItemResponse;
use crate::modules::todo::repository::TodosRepository;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};
use uuid::Uuid;

pub struct TodosService;

impl TodosService {
    pub async fn find(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<TodoItemResponse>, AppError> {
        let condition = Condition::all().add(TodosColumn::UserId.eq(user_id));
        let todos = TodosRepository::find(db, Some(condition)).await?;
        let todos = todos
            .into_iter()
            .map(|item| TodoItemResponse {
                id: item.id,
                title: item.title,
                completed: item.completed,
                created_at: item.created_at.to_string(),
            })
            .collect();

        Ok(todos)
    }

    pub async fn find_one(
        db: &DatabaseConnection,
        user_id: Uuid,
        id: i32,
    ) -> Result<TodoItemResponse, AppError> {
        let condition = Condition::all().add(TodosColumn::UserId.eq(user_id));
        let item = TodosRepository::find_one(db, id, Some(condition))
            .await?
            .ok_or(AppError::NotFound("Todo not found!".to_string()))?;

        let todo = TodoItemResponse {
            id: item.id,
            title: item.title,
            completed: item.completed,
            created_at: item.created_at.to_string(),
        };

        Ok(todo)
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: TodoCreateDto,
    ) -> Result<TodoItemResponse, AppError> {
        let todo = TodosRepository::create(db, user_id, dto).await?;
        let todo = TodoItemResponse {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            created_at: todo.created_at.to_string(),
        };

        Ok(todo)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        user_id: Uuid,
        dto: TodoUpdateDto,
    ) -> Result<TodoItemResponse, AppError> {
        let condition = Condition::all().add(TodosColumn::UserId.eq(user_id));
        let todo = TodosRepository::update(db, id, dto, Some(condition)).await?;
        let todo = TodoItemResponse {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            created_at: todo.created_at.to_string(),
        };

        Ok(todo)
    }

    pub async fn remove(db: &DatabaseConnection, id: i32, user_id: Uuid) -> Result<(), AppError> {
        let _ = TodosRepository::remove(db, id, user_id).await?;
        Ok(())
    }
}
