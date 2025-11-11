use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryName(String);

impl CategoryName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidCategoryName("Category name cannot be empty".to_string()));
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<CategoryName> for String {
    fn from(name: CategoryName) -> Self {
        name.0
    }
}

