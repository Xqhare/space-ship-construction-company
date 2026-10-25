use std::fmt::Display;

use aequa::{XffSerde, XffValue, xff};
use nemesis::{NemesisError, NemesisResultExt};

use crate::error::new_xff_serde_error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierType {
    Flat,
    Percentage,
}

impl XffSerde for ModifierType {
    fn to_xff(&self) -> XffValue {
        xff!(self.to_string())
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError> {
        if let Some(str) = xff.as_string() {
            match str.as_str() {
                "flat" => Ok(ModifierType::Flat),
                "percentage" => Ok(ModifierType::Percentage),
                _ => Err(new_xff_serde_error(
                    "Can't convert value to ModifierType, unknown modifier type.",
                ))
                .add_ctx(format!("Value: {:?}", xff)),
            }
        } else {
            Err(
                new_xff_serde_error("Can't convert value to ModifierType, not a string.")
                    .add_ctx(format!("Value: {:?}", xff)),
            )
        }
    }
}

impl Display for ModifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierType::Flat => write!(f, "flat"),
            ModifierType::Percentage => write!(f, "percentage"),
        }
    }
}
