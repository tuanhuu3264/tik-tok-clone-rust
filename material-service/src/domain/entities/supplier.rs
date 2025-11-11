use crate::domain::value_objects::{SupplierId, SupplierName, Email};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Supplier {
    pub id: SupplierId,
    pub name: SupplierName,
    pub email: Email,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Supplier {
    pub fn new(id: SupplierId, name: SupplierName, email: Email) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            email,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

