use crate::domain::value_objects::{PermissionId, PermissionName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: PermissionId,
    pub name: PermissionName,
    pub resource: String,
    pub action: String,
    pub created_at: DateTime<Utc>,
}

impl Permission {
    pub fn new(id: PermissionId, name: PermissionName, resource: String, action: String) -> Self {
        Self {
            id,
            name,
            resource,
            action,
            created_at: Utc::now(),
        }
    }
}

