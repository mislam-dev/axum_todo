use super::dto::{LoginResponse, LoginUserDto};
use super::jwt::{JwtPaylaod, create_jwt};
use crate::{
    core::errors::error::AppError,
    modules::user::{dto::UserCreateDto, password::verify_passwrod, service::UserService},
};
use sea_orm::DatabaseConnection;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        dto: LoginUserDto,
    ) -> Result<LoginResponse, AppError> {
        let user = UserService::find_by_email_with_password(db, &dto.email)
            .await
            .map_err(|_| AppError::BadRequest("Invalid Login credenditals".to_string()))?;

        let verify = verify_passwrod(&user.password, &dto.password)
            .await
            .map_err(|_| AppError::BadRequest("Invalid Login credenditals".to_string()))?;
        if !verify {
            return Err(AppError::BadRequest(
                "Invalid Login credenditals".to_string(),
            ));
        }
        let access_token = create_jwt(JwtPaylaod {
            email: user.email,
            sub: user.id,
        })
        .map_err(|_| {
            AppError::InternalServerError("Access token generation failed!".to_string())
        })?;

        Ok(LoginResponse {
            acess_token: access_token,
            refresh_token: "_".to_owned(),
        })
    }
    pub async fn register(db: &DatabaseConnection, dto: UserCreateDto) -> Result<(), AppError> {
        let _ = UserService::create(&db, dto).await?;

        Ok(())
    }
}
