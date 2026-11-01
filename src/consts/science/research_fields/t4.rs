use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum Disciplines {
    Engineering,
    CorporateLaw,
}

impl XffSerde for Disciplines {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Engineering => obj.insert("engineering", true),
            Self::CorporateLaw => obj.insert("corporate_law", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("engineering").is_some() {
                Ok(Disciplines::Engineering)
            } else if obj.get("corporate_law").is_some() {
                Ok(Disciplines::CorporateLaw)
            } else {
                Err(new_xff_serde_error("Unable to load Disciplines from XFF"))
            }
        } else {
            Err(new_xff_serde_error("Unable to load Disciplines from XFF"))
        }
    }
}
