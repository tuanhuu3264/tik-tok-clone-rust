use std::sync::Arc;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::SessionRepository;
use crate::domain::value_objects::{SessionId, Token};
use crate::domain::errors::DomainError;

pub struct LogoutUseCase<SR: SessionRepository> {
    session_repository: Arc<SR>,
}

impl<SR: SessionRepository> LogoutUseCase<SR> {
    pub fn new(session_repository: Arc<SR>) -> Self {
        Self { session_repository }
    }

    pub async fn execute(&self, token: &str) -> Result<(), ApplicationError> {
        let token = Token::new(token.to_string());
        let session = self.session_repository.find_by_token(&token).await?
            .ok_or(ApplicationError::Domain(DomainError::SessionNotFound))?;

        self.session_repository.delete(&session.id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

        Ok(())
    }
}

