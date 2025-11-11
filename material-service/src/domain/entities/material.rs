use crate::domain::value_objects::{MaterialId, MaterialName, MaterialCode, Unit};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Material {
    pub id: MaterialId,
    pub name: MaterialName,
    pub code: MaterialCode,
    pub material_type_id: Option<uuid::Uuid>,
    pub supplier_id: Option<uuid::Uuid>,
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
            material_type_id: None,
            supplier_id: None,
            unit,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn assign_to_type(&mut self, type_id: uuid::Uuid) {
        self.material_type_id = Some(type_id);
        self.updated_at = Utc::now();
    }

    pub fn assign_to_supplier(&mut self, supplier_id: uuid::Uuid) {
        self.supplier_id = Some(supplier_id);
        self.updated_at = Utc::now();
    }
}

