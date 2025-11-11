use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quantity(u32);

impl Quantity {
    pub fn new(quantity: u32) -> Self {
        Self(quantity)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<u32> for Quantity {
    fn from(quantity: u32) -> Self {
        Self(quantity)
    }
}

impl From<Quantity> for u32 {
    fn from(quantity: Quantity) -> Self {
        quantity.0
    }
}

