use super::entities::todos::Model as TodoModel;
use serde::{Deserialize, Serialize};

// Request DTOs
#[derive(Deserialize, Debug)]
pub struct TodoCreateDto {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct TodoUpdateDto {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TodoListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct IdParam(pub i32);

// --- Response DTOs

#[derive(Serialize)]
pub struct TodoItemResponse {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

impl From<TodoModel> for TodoItemResponse {
    fn from(model: TodoModel) -> Self {
        TodoItemResponse {
            id: model.id,
            title: model.title,
            completed: model.completed,
            created_at: model.created_at.to_string(),
        }
    }
}
