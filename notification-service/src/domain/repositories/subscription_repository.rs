use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::subscription::Subscription;
use crate::domain::value_objects::{SubscriptionId, UserId, Channel};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait SubscriptionRepository: Send + Sync {
    async fn create(&self, subscription: &Subscription) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &SubscriptionId) -> Result<Option<Subscription>, DomainError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Subscription>, DomainError>;
    async fn find_by_user_and_channel(&self, user_id: &UserId, channel: &Channel) -> Result<Option<Subscription>, DomainError>;
    async fn update(&self, subscription: &Subscription) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: SubscriptionRepository> SubscriptionRepository for Arc<R> {
    async fn create(&self, subscription: &Subscription) -> Result<(), DomainError> {
        (**self).create(subscription).await
    }

    async fn find_by_id(&self, id: &SubscriptionId) -> Result<Option<Subscription>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Subscription>, DomainError> {
        (**self).find_by_user_id(user_id).await
    }

    async fn find_by_user_and_channel(&self, user_id: &UserId, channel: &Channel) -> Result<Option<Subscription>, DomainError> {
        (**self).find_by_user_and_channel(user_id, channel).await
    }

    async fn update(&self, subscription: &Subscription) -> Result<(), DomainError> {
        (**self).update(subscription).await
    }
}

