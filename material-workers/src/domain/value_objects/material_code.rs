use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialCode(String);

impl MaterialCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for MaterialCode {
    fn from(code: String) -> Self {
        Self(code)
    }
}

