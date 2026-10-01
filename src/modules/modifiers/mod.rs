use std::fmt::Display;

use aequa::{Object, XffValue, hp_float::HpFloat, xff};

#[derive(Debug, Clone)]
pub struct Modifier {
    pub impacted_base_value_name: String,
    pub kind: ModifierType,
    pub value: HpFloat,
}

impl From<Object> for Modifier {
    fn from(value: Object) -> Self {
        let impacted_base_value_name = value
            .get("impacted_base_value_name")
            .unwrap()
            .into_string()
            .expect("Impacted base value name is not a string");
        let kind = value
            .get("kind")
            .unwrap()
            .into_string()
            .expect("Modifier Kind is not a string")
            .into();
        let value = value
            .get("value")
            .unwrap()
            .into_hp_float()
            .expect("Modifier value is not a number");
        Modifier {
            impacted_base_value_name,
            kind,
            value,
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
