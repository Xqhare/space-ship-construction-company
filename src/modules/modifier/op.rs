use aequa::{XffSerde, XffValue, xff};
use nemesis::{NemesisError, NemesisResultExt};

use crate::error::{new_startup_error, new_xff_serde_error};
#[derive(Debug, Clone)]
pub enum ModifierOp {
    Add,
    Sub,
    Mul,
    Div,
    Set,
    Lim,
}

impl XffSerde for ModifierOp {
    fn to_xff(&self) -> XffValue {
        let op = match self {
            ModifierOp::Add => "add",
            ModifierOp::Sub => "sub",
            ModifierOp::Mul => "mul",
            ModifierOp::Div => "div",
            ModifierOp::Set => "set",
            ModifierOp::Lim => "lim",
        };
        xff!(op)
    }
    fn from_xff(xff: &XffValue) -> Result<ModifierOp, NemesisError> {
        if let Some(str) = xff.as_string() {
            Ok(match str.as_str() {
                "add" => ModifierOp::Add,
                "set" => ModifierOp::Set,
                "sub" => ModifierOp::Sub,
                "mul" => ModifierOp::Mul,
                "div" => ModifierOp::Div,
                "lim" => ModifierOp::Lim,
                _ => {
                    return Err(new_xff_serde_error(
                        "Can't convert value to ModifierOp, unknown ModifierOp",
                    ))
                    .add_ctx(format!("Value: '{:?}'", xff));
                }
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to ModifierOp, not a String.",
            ))
            .add_ctx(format!("Value: '{:?}'", xff))
        }
    }
}
