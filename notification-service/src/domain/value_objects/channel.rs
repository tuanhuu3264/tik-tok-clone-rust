use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    Email,
    SMS,
    Push,
    InApp,
}

impl Channel {
    pub fn as_str(&self) -> &str {
        match self {
            Channel::Email => "email",
            Channel::SMS => "sms",
            Channel::Push => "push",
            Channel::InApp => "in_app",
        }
    }
}

