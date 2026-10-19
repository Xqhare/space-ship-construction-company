use std::fmt::Display;

use aequa::{Object, XffValue, hp_float::HpFloat, xff};
use nemesis::NemesisResultExt;

use crate::error::{SSCCResult, new_startup_error};

#[derive(Debug, Clone)]
pub struct Modifier {
    pub impacted_base_value_name: String,
    pub kind: ModifierType,
    pub value: HpFloat,
}

impl Modifier {
    pub fn to_xff(&self) -> XffValue {
        xff!(Into::<Object>::into(self.clone()))
    }
    pub fn from_xff(value: &XffValue) -> SSCCResult<Modifier> {
        if let Some(obj) = value.as_object()
            && let Some(ibvm) = obj.get("impacted_base_value_name")
            && let Some(ibvm) = ibvm.into_string()
            && let Some(k) = obj.get("kind")
            && let Some(k) = k.into_string()
            && let Some(v) = obj.get("value")
            && let Some(v) = v.into_hp_float()
        {
            Ok(Modifier {
                impacted_base_value_name: ibvm,
                kind: k.into(),
                value: v,
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to Modifier, not an object.",
            ))
            .add_ctx(format!("Value: '{:?}'", value))
        }
    }
}

impl Into<Object> for Modifier {
    fn into(self) -> Object {
        Object::from(vec![
            (
                "impacted_base_value_name",
                xff!(self.impacted_base_value_name),
            ),
            ("kind", xff!(self.kind.to_string())),
            ("value", xff!(self.value)),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierType {
    Flat,
    Percentage,
}

impl From<&str> for ModifierType {
    fn from(value: &str) -> Self {
        match value {
            "flat" => ModifierType::Flat,
            "percentage" => ModifierType::Percentage,
            // Lets just fall back to percentage instead of panic
            _ => ModifierType::Percentage,
        }
    }
}

impl From<String> for ModifierType {
    fn from(value: String) -> Self {
        value.as_str().into()
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

#[test]
fn from_and_into() {
    let modifier_type: ModifierType = "flat".into();
    let modifier_type_string: String = modifier_type.to_string();
    assert_eq!(modifier_type_string, "flat");
    assert_eq!(modifier_type, ModifierType::Flat);

    let modifier_type: ModifierType = "percentage".into();
    let modifier_type_string: String = modifier_type.to_string();
    assert_eq!(modifier_type_string, "percentage");
    assert_eq!(modifier_type, ModifierType::Percentage);
}
