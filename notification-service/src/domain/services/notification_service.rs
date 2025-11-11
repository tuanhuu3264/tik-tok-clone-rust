use std::sync::Arc;
use crate::domain::repositories::SubscriptionRepository;
use crate::domain::value_objects::{UserId, Channel};
use crate::domain::errors::DomainError;

pub struct NotificationDomainService<R: SubscriptionRepository> {
    subscription_repository: Arc<R>,
}

impl<R: SubscriptionRepository> NotificationDomainService<R> {
    pub fn new(subscription_repository: Arc<R>) -> Self {
        Self { subscription_repository }
    }

    pub async fn is_user_subscribed(&self, user_id: &UserId, channel: &Channel) -> Result<bool, DomainError> {
        let subscription = self.subscription_repository
            .find_by_user_and_channel(user_id, channel)
            .await?;
        
        Ok(subscription.map(|s| s.is_active).unwrap_or(false))
    }

    pub async fn can_send_notification(&self, user_id: &UserId, channel: &Channel) -> Result<bool, DomainError> {
        self.is_user_subscribed(user_id, channel).await
    }
}

