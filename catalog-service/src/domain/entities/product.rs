use crate::domain::value_objects::{ProductId, ProductName, SKU, Price, Description};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Product {
    pub id: ProductId,
    pub name: ProductName,
    pub sku: SKU,
    pub description: Description,
    pub price: Price,
    pub category_id: Option<uuid::Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        sku: SKU,
        description: Description,
        price: Price,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            sku,
            description,
            price,
            category_id: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn assign_to_category(&mut self, category_id: uuid::Uuid) {
        self.category_id = Some(category_id);
        self.updated_at = Utc::now();
    }

    pub fn update_price(&mut self, new_price: Price) {
        self.price = new_price;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}

