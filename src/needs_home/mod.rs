use aequa::{Object, XffSerde, XffValue, xff};
use athena::float::HpFloat;
use nemesis::NemesisError;

use crate::error::new_xff_serde_error;

/// A pair of base value name and value
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BaseValuePair {
    pub base_value_name: String,
    pub base_value_value: HpFloat,
}

impl BaseValuePair {
    pub fn new<S: Into<String>>(name: S, value: HpFloat) -> Self {
        Self {
            base_value_name: name.into(),
            base_value_value: value,
        }
    }
    pub fn as_parts(&self) -> (&str, &HpFloat) {
        (&self.base_value_name, &self.base_value_value)
    }
}

impl XffSerde for BaseValuePair {
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(name) = obj.get("base_value_name")
            && let Some(name) = name.into_string()
            && let Some(value) = obj.get("base_value_value")
            && let Some(value) = value.into_hp_float()
        {
            Ok(BaseValuePair {
                base_value_name: name,
                base_value_value: value,
            })
        } else {
            Err(new_xff_serde_error(
                "Can't convert value to BaseValuePair, not an object or doesn't have expected keys.",
            )
            .add_ctx(format!("Value: {:?}", xff)))
        }
    }
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("base_value_name", self.base_value_name.clone());
        obj.insert("base_value_value", self.base_value_value);
        xff!(obj)
    }
}
