use jsonwebtoken::Header;
use migration::prelude::chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::errors::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub struct JwtPaylaod {
    pub sub: Uuid,
    pub email: String,
}

pub fn create_jwt(data: JwtPaylaod) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

    let now = Utc::now();
    let exp: usize = (now + Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = Claims {
        sub: data.sub,
        exp,
        iat,
        email: data.email.to_owned(),
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| AppError::Config(format!("JWT encoding failed: {}", err)))
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

    let token_data = jsonwebtoken::decode(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|err| AppError::Config(format!("JWT encoding failed: {}", err)))?;

    Ok(token_data.claims)
}
