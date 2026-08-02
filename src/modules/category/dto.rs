use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct CategoryCreateDto {
    #[validate(length(min = 1, message = "This field is required!"))]
    pub name: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct CategoryUpdateDto {
    #[validate(length(min = 1, message = "This field is required!"))]
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct IdParam(pub Uuid);

// --- Response DTOs

#[derive(Serialize)]
pub struct CategoryItemResponse {
    pub id: Uuid,
    pub name: String,
}
