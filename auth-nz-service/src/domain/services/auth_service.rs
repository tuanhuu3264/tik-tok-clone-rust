use std::sync::Arc;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use crate::domain::errors::DomainError;

pub struct AuthService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn is_email_taken(&self, email: &Email) -> Result<bool, DomainError> {
        let existing_user = self.user_repository.find_by_email(email).await?;
        Ok(existing_user.is_some())
    }

    pub async fn validate_user_creation(&self, email: &Email) -> Result<(), DomainError> {
        if self.is_email_taken(email).await? {
            return Err(DomainError::UserAlreadyExists);
        }
        Ok(())
    }
}

