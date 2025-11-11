use std::sync::Arc;
use crate::application::dto::{RegisterDto, AuthResponseDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{UserRepository, SessionRepository};
use crate::domain::services::AuthService;
use crate::domain::value_objects::{Email, PasswordHash, UserId, SessionId, Token};
use crate::domain::entities::user::User;
use crate::domain::entities::session::Session;
use chrono::{Duration, Utc};

pub struct RegisterUseCase<UR: UserRepository, SR: SessionRepository> {
    user_repository: Arc<UR>,
    session_repository: Arc<SR>,
    auth_service: AuthService<UR>,
}

impl<UR: UserRepository, SR: SessionRepository> RegisterUseCase<UR, SR> {
    pub fn new(user_repository: Arc<UR>, session_repository: Arc<SR>) -> Self {
        let auth_service = AuthService::new(Arc::clone(&user_repository));
        Self {
            user_repository,
            session_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, dto: RegisterDto) -> Result<AuthResponseDto, ApplicationError> {
        let email = Email::new(dto.email)?;
        
        self.auth_service.validate_user_creation(&email).await?;

        // In real implementation, hash password using argon2
        let password_hash = PasswordHash::new(dto.password); // TODO: hash password
        
        let user = User::new(UserId::new(), email, password_hash);
        self.user_repository.create(&user).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

        // Create session
        let token = Token::new(uuid::Uuid::new_v4().to_string()); // TODO: generate JWT
        let expires_at = Utc::now() + Duration::hours(24);
        let session = Session::new(SessionId::new(), user.id, token.clone(), expires_at);
        
        self.session_repository.create(&session).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

        Ok(AuthResponseDto {
            token: token.as_str().to_string(),
            expires_at: expires_at.to_rfc3339(),
        })
    }
}

