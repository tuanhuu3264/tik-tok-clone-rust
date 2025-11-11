use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    Kilogram,
    Gram,
    Liter,
    Milliliter,
    Piece,
    Meter,
    Centimeter,
}

impl Unit {
    pub fn as_str(&self) -> &str {
        match self {
            Unit::Kilogram => "kg",
            Unit::Gram => "g",
            Unit::Liter => "L",
            Unit::Milliliter => "mL",
            Unit::Piece => "pcs",
            Unit::Meter => "m",
            Unit::Centimeter => "cm",
        }
    }
}

