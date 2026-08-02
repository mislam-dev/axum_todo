use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct UserCreateDto {
    #[validate(length(min = 1, message = "Name is required!"))]
    pub name: String,

    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateDto {
    #[validate(length(min = 1, message = "Name is required!"))]
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct IdParam(pub Uuid);

// --- Response DTOs

#[derive(Serialize)]
pub struct UserItemResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
#[derive(Serialize)]
pub struct UserItemWithPassword {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
