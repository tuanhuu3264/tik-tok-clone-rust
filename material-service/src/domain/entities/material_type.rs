use crate::domain::value_objects::{MaterialTypeId, MaterialTypeName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MaterialType {
    pub id: MaterialTypeId,
    pub name: MaterialTypeName,
    pub created_at: DateTime<Utc>,
}

impl MaterialType {
    pub fn new(id: MaterialTypeId, name: MaterialTypeName) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
        }
    }
}

