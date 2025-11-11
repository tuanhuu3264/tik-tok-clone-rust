use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidRoleName("Role name cannot be empty".to_string()));
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<RoleName> for String {
    fn from(name: RoleName) -> Self {
        name.0
    }
}

