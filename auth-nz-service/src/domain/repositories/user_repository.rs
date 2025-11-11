use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::user::User;
use crate::domain::value_objects::{Email, UserId};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn update(&self, user: &User) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: UserRepository> UserRepository for Arc<R> {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        (**self).create(user).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        (**self).find_by_email(email).await
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        (**self).update(user).await
    }
}

