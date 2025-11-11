use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Price(Decimal);

impl Price {
    pub fn new(amount: Decimal) -> Self {
        Self(amount)
    }

    pub fn from_cents(cents: i64) -> Self {
        Self(Decimal::new(cents, 2))
    }

    pub fn value(&self) -> Decimal {
        self.0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > Decimal::ZERO
    }
}

impl From<Decimal> for Price {
    fn from(amount: Decimal) -> Self {
        Self(amount)
    }
}

impl From<Price> for Decimal {
    fn from(price: Price) -> Self {
        price.0
    }
}

