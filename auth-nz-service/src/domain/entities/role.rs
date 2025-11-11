use crate::domain::value_objects::{RoleId, RoleName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(id: RoleId, name: RoleName) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

