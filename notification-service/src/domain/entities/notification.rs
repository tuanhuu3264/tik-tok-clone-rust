use crate::domain::value_objects::{NotificationId, UserId, Channel, Message, Title};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub channel: Channel,
    pub title: Title,
    pub message: Message,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    pub fn new(
        id: NotificationId,
        user_id: UserId,
        channel: Channel,
        title: Title,
        message: Message,
    ) -> Self {
        Self {
            id,
            user_id,
            channel,
            title,
            message,
            is_read: false,
            created_at: Utc::now(),
            read_at: None,
        }
    }

    pub fn mark_as_read(&mut self) {
        if !self.is_read {
            self.is_read = true;
            self.read_at = Some(Utc::now());
        }
    }
}

