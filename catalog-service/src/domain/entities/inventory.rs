use crate::domain::value_objects::{ProductId, Quantity};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Inventory {
    pub product_id: ProductId,
    pub quantity: Quantity,
    pub reserved_quantity: Quantity,
    pub updated_at: DateTime<Utc>,
}

impl Inventory {
    pub fn new(product_id: ProductId, quantity: Quantity) -> Self {
        Self {
            product_id,
            quantity,
            reserved_quantity: Quantity::zero(),
            updated_at: Utc::now(),
        }
    }

    pub fn available_quantity(&self) -> Quantity {
        Quantity::new(self.quantity.value() - self.reserved_quantity.value())
    }

    pub fn reserve(&mut self, amount: u32) -> Result<(), crate::domain::errors::DomainError> {
        let available = self.available_quantity().value();
        if amount > available {
            return Err(crate::domain::errors::DomainError::InsufficientStock);
        }
        self.reserved_quantity = Quantity::new(self.reserved_quantity.value() + amount);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn release(&mut self, amount: u32) {
        if amount <= self.reserved_quantity.value() {
            self.reserved_quantity = Quantity::new(self.reserved_quantity.value() - amount);
            self.updated_at = Utc::now();
        }
    }
}

