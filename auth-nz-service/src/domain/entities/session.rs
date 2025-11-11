use crate::domain::value_objects::{SessionId, UserId, Token};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub token: Token,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(id: SessionId, user_id: UserId, token: Token, expires_at: DateTime<Utc>) -> Self {
        Self {
            id,
            user_id,
            token,
            expires_at,
            created_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

