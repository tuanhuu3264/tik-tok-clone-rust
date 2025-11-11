use crate::domain::value_objects::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Password {
    pub user_id: UserId,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Password {
    pub fn new(user_id: UserId, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_password_hash(&mut self, new_hash: String) {
        self.password_hash = new_hash;
        self.updated_at = Utc::now();
    }

    pub fn verify(&self, password: &str) -> bool {
        self.password_hash == password
    }
}
