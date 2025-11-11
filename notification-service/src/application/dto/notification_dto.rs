use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateNotificationDto {
    pub user_id: String,
    pub channel: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationDto {
    pub id: String,
    pub user_id: String,
    pub channel: String,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: String,
}

