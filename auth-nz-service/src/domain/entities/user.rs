use crate::domain::value_objects::{Email, UserId, PasswordHash};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            id,
            email,
            password_hash,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn update_password(&mut self, new_hash: PasswordHash) {
        self.password_hash = new_hash;
        self.updated_at = Utc::now();
    }
}

