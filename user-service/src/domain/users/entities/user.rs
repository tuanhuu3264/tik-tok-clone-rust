use crate::domain::value_objects::{Email, UserId, Username};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, username: Username, email: Email) -> Self {
        let now = Utc::now();
        Self {
            id,
            username,
            email,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_email(&mut self, new_email: Email) {
        self.email = new_email;
        self.updated_at = Utc::now();
    }

    pub fn update_username(&mut self, new_username: Username) {
        self.username = new_username;
        self.updated_at = Utc::now();
    }
}

