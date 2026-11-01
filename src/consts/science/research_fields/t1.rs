use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum Formal {
    /// Thinking about logic / numbers
    Mathematics,
    /// Thinking about the human condition
    Philosophy,
    /// Thinking about the physical world
    Empiricism,
}

impl XffSerde for Formal {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Mathematics => obj.insert("mathematics", true),
            Self::Philosophy => obj.insert("philosophy", true),
            Self::Empiricism => obj.insert("empiricism", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("mathematics").is_some() {
                Ok(Formal::Mathematics)
            } else if obj.get("philosophy").is_some() {
                Ok(Formal::Philosophy)
            } else if obj.get("empiricism").is_some() {
                Ok(Formal::Empiricism)
            } else {
                Err(new_xff_serde_error("Unable to load Formal from XFF"))
            }
        } else {
            Err(new_xff_serde_error("Unable to load Formal from XFF"))
        }
    }
}
