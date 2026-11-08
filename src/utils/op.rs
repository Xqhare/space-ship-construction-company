use aequa::{XffSerde, XffValue, xff};
use nemesis::{NemesisError, NemesisResultExt};

use crate::error::{new_startup_error, new_xff_serde_error};
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Set,
    Lim,
}

impl Default for Op {
    fn default() -> Self {
        Op::Add
    }
}

impl XffSerde for Op {
    fn to_xff(&self) -> XffValue {
        let op = match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Div => "div",
            Op::Set => "set",
            Op::Lim => "lim",
        };
        xff!(op)
    }
    fn from_xff(xff: &XffValue) -> Result<Op, NemesisError> {
        if let Some(str) = xff.as_string() {
            Ok(match str.as_str() {
                "add" => Op::Add,
                "set" => Op::Set,
                "sub" => Op::Sub,
                "mul" => Op::Mul,
                "div" => Op::Div,
                "lim" => Op::Lim,
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
