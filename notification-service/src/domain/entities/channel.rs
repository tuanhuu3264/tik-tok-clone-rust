use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    SMS,
    Push,
    InApp,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationChannel::Email => "email",
            NotificationChannel::SMS => "sms",
            NotificationChannel::Push => "push",
            NotificationChannel::InApp => "in_app",
        }
    }
}

