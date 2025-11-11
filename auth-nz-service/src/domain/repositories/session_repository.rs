use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::session::Session;
use crate::domain::value_objects::{SessionId, Token, UserId};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &Session) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &SessionId) -> Result<Option<Session>, DomainError>;
    async fn find_by_token(&self, token: &Token) -> Result<Option<Session>, DomainError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, DomainError>;
    async fn delete(&self, id: &SessionId) -> Result<(), DomainError>;
    async fn delete_by_user_id(&self, user_id: &UserId) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: SessionRepository> SessionRepository for Arc<R> {
    async fn create(&self, session: &Session) -> Result<(), DomainError> {
        (**self).create(session).await
    }

    async fn find_by_id(&self, id: &SessionId) -> Result<Option<Session>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_token(&self, token: &Token) -> Result<Option<Session>, DomainError> {
        (**self).find_by_token(token).await
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, DomainError> {
        (**self).find_by_user_id(user_id).await
    }

    async fn delete(&self, id: &SessionId) -> Result<(), DomainError> {
        (**self).delete(id).await
    }

    async fn delete_by_user_id(&self, user_id: &UserId) -> Result<(), DomainError> {
        (**self).delete_by_user_id(user_id).await
    }
}

