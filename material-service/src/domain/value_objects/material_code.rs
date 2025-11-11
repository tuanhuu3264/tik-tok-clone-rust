use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialCode(String);

impl MaterialCode {
    pub fn new(code: String) -> Result<Self, DomainError> {
        if code.is_empty() {
            return Err(DomainError::InvalidMaterialCode("Material code cannot be empty".to_string()));
        }
        Ok(Self(code))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<MaterialCode> for String {
    fn from(code: MaterialCode) -> Self {
        code.0
    }
}

