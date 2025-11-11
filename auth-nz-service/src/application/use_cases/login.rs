use std::sync::Arc;
use crate::application::dto::{LoginDto, AuthResponseDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{UserRepository, SessionRepository};
use crate::domain::value_objects::{Email, PasswordHash, SessionId, Token};
use crate::domain::entities::session::Session;
use crate::domain::errors::DomainError;
use chrono::{Duration, Utc};

pub struct LoginUseCase<UR: UserRepository, SR: SessionRepository> {
    user_repository: Arc<UR>,
    session_repository: Arc<SR>,
}

impl<UR: UserRepository, SR: SessionRepository> LoginUseCase<UR, SR> {
    pub fn new(user_repository: Arc<UR>, session_repository: Arc<SR>) -> Self {
        Self {
            user_repository,
            session_repository,
        }
    }

    pub async fn execute(&self, dto: LoginDto) -> Result<AuthResponseDto, ApplicationError> {
        let email = Email::new(dto.email)?;
        
        let user = self.user_repository.find_by_email(&email).await?
            .ok_or(ApplicationError::Domain(DomainError::UserNotFound))?;

        if !user.is_active {
            return Err(ApplicationError::Domain(DomainError::Unauthorized));
        }

        // TODO: Verify password hash
        // For now, just check if password matches (in real implementation, use argon2)
        if user.password_hash.as_str() != dto.password {
            return Err(ApplicationError::Domain(DomainError::InvalidPassword));
        }

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

