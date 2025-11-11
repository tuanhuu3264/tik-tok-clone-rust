use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SKU(String);

impl SKU {
    pub fn new(sku: String) -> Result<Self, DomainError> {
        if sku.is_empty() {
            return Err(DomainError::InvalidSKU("SKU cannot be empty".to_string()));
        }
        if sku.len() > 50 {
            return Err(DomainError::InvalidSKU("SKU too long".to_string()));
        }
        Ok(Self(sku))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<SKU> for String {
    fn from(sku: SKU) -> Self {
        sku.0
    }
}

