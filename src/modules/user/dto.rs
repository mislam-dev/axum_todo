use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct UserCreateDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateDto {
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
