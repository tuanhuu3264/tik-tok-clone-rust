use crate::domain::value_objects::{MaterialId, MaterialName, MaterialCode, Unit};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Material {
    pub id: MaterialId,
    pub name: MaterialName,
    pub code: MaterialCode,
    pub unit: Unit,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Material {
    pub fn new(
        id: MaterialId,
        name: MaterialName,
        code: MaterialCode,
        unit: Unit,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            code,
            unit,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

