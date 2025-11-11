use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialTypeName(String);

impl MaterialTypeName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidMaterialTypeName("Material type name cannot be empty".to_string()));
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<MaterialTypeName> for String {
    fn from(name: MaterialTypeName) -> Self {
        name.0
    }
}

