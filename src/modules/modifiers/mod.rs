use std::{fmt::Display, time::Duration};

use aequa::{Object, XffSerde, XffValue, hp_float::HpFloat, xff};
use nemesis::{NemesisError, NemesisResultExt};

use crate::{
    error::{SSCCResult, new_startup_error},
    utils::boxes::flag_box::Flag,
};

#[derive(Debug, Clone)]
pub struct Modifier {
    pub impacted_base_value_name: String,
    pub kind: ModifierType,
    pub duration: Option<Duration>,
    pub requires_flag: Option<Flag>,
    pub op: ModifierOp,
}

#[derive(Debug, Clone)]
pub enum ModifierOp {
    Add(HpFloat),
    Set(HpFloat),
    Sub(HpFloat),
    Mul(HpFloat),
    Div(HpFloat),
}

impl XffSerde for ModifierOp {
    fn to_xff(&self) -> XffValue {
        let (op, value) = match self {
            ModifierOp::Add(v) => ("add", v),
            ModifierOp::Set(v) => ("set", v),
            ModifierOp::Sub(v) => ("sub", v),
            ModifierOp::Mul(v) => ("mul", v),
            ModifierOp::Div(v) => ("div", v),
        };
        let mut obj = Object::new();
        obj.insert("op", op);
        obj.insert("value", XffValue::HpFloat(*value));
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<ModifierOp, NemesisError> {
        if let Some(obj) = xff.as_object()
            && let Some(op) = obj.get("op")
            && let Some(op) = op.into_string()
            && let Some(v) = obj.get("value")
            && let Some(v) = v.into_hp_float()
        {
            Ok(match op.as_str() {
                "add" => ModifierOp::Add(v),
                "set" => ModifierOp::Set(v),
                "sub" => ModifierOp::Sub(v),
                "mul" => ModifierOp::Mul(v),
                "div" => ModifierOp::Div(v),
                _ => return Err(new_startup_error("Unknown ModifierOp")),
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to ModifierOp, not an object or doesn't have expected keys.",
            ))
            .add_ctx(format!("Value: '{:?}'", xff))
        }
    }
}

impl XffSerde for Modifier {
    fn to_xff(&self) -> XffValue {
        xff!(Into::<Object>::into(self.clone()))
    }
    fn from_xff(value: &XffValue) -> SSCCResult<Modifier> {
        if let Some(obj) = value.as_object()
            && let Some(ibvm) = obj.get("impacted_base_value_name")
            && let Some(ibvm) = ibvm.into_string()
            && let Some(k) = obj.get("kind")
            && let Some(k) = k.into_string()
            && let Some(op) = obj.get("op")
        {
            let op = ModifierOp::from_xff(op)?;
            let duration = {
                if let Some(dur) = obj.get("duration") {
                    if let Some(dur) = dur.into_std_duration() {
                        Some(dur)
                    } else {
                        return Err(new_startup_error(
                            "Duration key exists but can't convert duration to std::time::Duration",
                        ))
                        .add_ctx(format!("Duration: '{:?}'", dur));
                    }
                } else {
                    None
                }
            };
            Ok(Modifier {
                impacted_base_value_name: ibvm,
                kind: k.into(),
                duration,
                op,
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
        let mut obj = Object::new();
        obj.insert("impacted_base_value_name", self.impacted_base_value_name);
        obj.insert("kind", self.kind.to_string());
        if let Some(dur) = self.duration {
            obj.insert("duration", dur);
        }
        obj.insert("op", self.op.to_xff());
        obj
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
