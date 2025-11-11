use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Token {
    fn from(token: String) -> Self {
        Self(token)
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        token.0
    }
}

