use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct CategoryCreateDto {
    #[validate(length(min = 1, message = "This field is requried!"))]
    pub name: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct CategoryUpdateDto {
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CategoryListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct IdParam(pub Uuid);

// --- Response DTOs

#[derive(Serialize)]
pub struct CategoryItemResponse {
    pub id: Uuid,
    pub name: String,
}
