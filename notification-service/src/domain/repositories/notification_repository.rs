use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::notification::Notification;
use crate::domain::value_objects::{NotificationId, UserId};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, notification: &Notification) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &NotificationId) -> Result<Option<Notification>, DomainError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Notification>, DomainError>;
    async fn update(&self, notification: &Notification) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: NotificationRepository> NotificationRepository for Arc<R> {
    async fn create(&self, notification: &Notification) -> Result<(), DomainError> {
        (**self).create(notification).await
    }

    async fn find_by_id(&self, id: &NotificationId) -> Result<Option<Notification>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Notification>, DomainError> {
        (**self).find_by_user_id(user_id).await
    }

    async fn update(&self, notification: &Notification) -> Result<(), DomainError> {
        (**self).update(notification).await
    }
}

