use serde::{Deserialize, Serialize};
use validator::Validate;

// Request DTOs
#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserDto {
    #[validate(email(message = "This must be an valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
// Response DTOs
#[derive(Serialize, Debug, Validate)]
pub struct LoginResponse {
    pub acess_token: String,
    pub refresh_token: String,
}
