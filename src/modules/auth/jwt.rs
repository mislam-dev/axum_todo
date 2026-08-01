use jsonwebtoken::Header;
use migration::prelude::chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"your-super-secret-jwt-key"; // In prod, load from std::env

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub struct JwtPaylaod {
    pub sub: String,
    pub email: String,
}

pub fn create_jwt(data: JwtPaylaod) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp: usize = (now + Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = Claims {
        sub: data.sub.to_owned(),
        exp,
        iat,
        email: data.email.to_owned(),
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(SECRET),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = jsonwebtoken::decode(
        token,
        &jsonwebtoken::DecodingKey::from_secret(SECRET),
        &jsonwebtoken::Validation::default(),
    )?;

    Ok(token_data.claims)
}
