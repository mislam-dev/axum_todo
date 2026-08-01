use super::jwt::{Claims, verify_jwt};
use axum::http::header::AUTHORIZATION;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        verify_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)
    }
}
