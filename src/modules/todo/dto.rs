use super::entities::todos::Model as TodoModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct TodoCreateDto {
    #[validate(length(min = 1, message = "title is required!"))]
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct TodoUpdateDto {
    #[validate(length(min = 1, message = "title is required!"))]
    pub title: Option<String>,
    pub completed: Option<bool>,
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
