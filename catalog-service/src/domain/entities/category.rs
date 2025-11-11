use crate::domain::value_objects::{CategoryId, CategoryName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub name: CategoryName,
    pub parent_id: Option<uuid::Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub fn new(id: CategoryId, name: CategoryName) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            parent_id: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_parent(&mut self, parent_id: uuid::Uuid) {
        self.parent_id = Some(parent_id);
        self.updated_at = Utc::now();
    }
}

