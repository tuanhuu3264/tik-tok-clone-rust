use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SupplierName(String);

impl SupplierName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidSupplierName("Supplier name cannot be empty".to_string()));
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<SupplierName> for String {
    fn from(name: SupplierName) -> Self {
        name.0
    }
}

