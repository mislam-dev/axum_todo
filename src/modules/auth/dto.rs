use serde::{Deserialize, Serialize};

// Request DTOs
#[derive(Deserialize, Debug)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}
// Response DTOs
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub acess_token: String,
    pub refresh_token: String,
}
