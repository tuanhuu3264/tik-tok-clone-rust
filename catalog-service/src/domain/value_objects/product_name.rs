use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidProductName("Product name cannot be empty".to_string()));
        }
        if name.len() > 200 {
            return Err(DomainError::InvalidProductName("Product name too long".to_string()));
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<ProductName> for String {
    fn from(name: ProductName) -> Self {
        name.0
    }
}

