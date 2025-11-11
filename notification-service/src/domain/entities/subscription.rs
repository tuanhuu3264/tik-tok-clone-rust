use crate::domain::value_objects::{SubscriptionId, UserId, Channel};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub user_id: UserId,
    pub channel: Channel,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, user_id: UserId, channel: Channel) -> Self {
        Self {
            id,
            user_id,
            channel,
            is_active: true,
            created_at: Utc::now(),
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
}

