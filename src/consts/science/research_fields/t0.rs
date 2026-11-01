use aequa::{XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum Logic {
    Logic,
}

impl XffSerde for Logic {
    fn to_xff(&self) -> XffValue {
        xff!("logic")
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(str) = xff.into_string()
            && str == "logic"
        {
            Ok(Logic::Logic)
        } else {
            Err(new_xff_serde_error("Unable to load Logic from XFF"))
        }
    }
}
