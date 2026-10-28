use std::time::Duration;

use aequa::{Array, Object, XffSerde, XffValue, xff};
use horae::Utc;
use nemesis::NemesisResultExt;

use crate::{
    error::{SSCCResult, new_xff_serde_error},
    modules::modifier::{op::ModifierOp, r#type::ModifierType},
    needs_home::BaseValuePair,
    utils::boxes::data_box::flag_box::Flag,
};

pub mod op;
pub mod r#type;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ModifierWithEndUtc {
    pub modifier: Modifier,
    pub end_utc: Option<Utc>,
}

impl ModifierWithEndUtc {
    pub fn new(modifier: Modifier, now_utc: Utc) -> ModifierWithEndUtc {
        let end_utc = if let Some(dur) = modifier.duration {
            Some(now_utc + dur)
        } else {
            None
        };
        ModifierWithEndUtc { modifier, end_utc }
    }
    pub fn new_no_end(modifier: Modifier) -> ModifierWithEndUtc {
        ModifierWithEndUtc {
            modifier,
            end_utc: None,
        }
    }
}

impl XffSerde for ModifierWithEndUtc {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("modifier", self.modifier.to_xff());
        if let Some(utc) = &self.end_utc {
            obj.insert("end_utc", utc.to_xff());
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(modifier) = obj.get("modifier")
        {
            let end_utc = if let Some(end_utc) = obj.get("end_utc") {
                Some(Utc::from_xff(end_utc)?)
            } else {
                None
            };
            let modifier = Modifier::from_xff(modifier)
                .add_ctx("While parsing ModifierWithEndUtc helper type.")?;
            Ok(ModifierWithEndUtc { modifier, end_utc })
        } else {
            Err(new_xff_serde_error(
                "Can't convert value to ModifierWithEndUtc, not an object or doesn't have expected keys.",
            )
            .add_ctx(format!("Value: {xff:?}")))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Modifier {
    pub impacted_base_value_pairs: Vec<BaseValuePair>,
    pub kind: ModifierType,
    pub duration: Option<Duration>,
    pub adds_flags: Option<Vec<Flag>>,
    pub required_flags: Option<Vec<Flag>>,
    pub op: ModifierOp,
}

impl Modifier {
    pub fn get_pairs(&self, base_value_name: &str) -> Vec<BaseValuePair> {
        self.impacted_base_value_pairs
            .iter()
            .filter(|bvp| bvp.base_value_name == base_value_name)
            .cloned()
            .collect()
    }
}

impl XffSerde for Modifier {
    fn to_xff(&self) -> XffValue {
        let impacted_base_value_pairs = {
            let mut tmp_ary = Array::with_capacity(self.impacted_base_value_pairs.len());
            for bvp in &self.impacted_base_value_pairs {
                tmp_ary.push(bvp.to_xff());
            }
            tmp_ary
        };
        let kind = self.kind.to_xff();
        let op = self.op.to_xff();
        let mut obj = Object::new();
        obj.insert("impacted_base_value_pairs", impacted_base_value_pairs);
        obj.insert("kind", kind);
        obj.insert("op", op);
        if let Some(dur) = &self.duration {
            obj.insert("duration", *dur);
        }
        if let Some(adds_flags) = &self.adds_flags {
            let mut flags_ary = Array::with_capacity(adds_flags.len());
            for flag in adds_flags {
                flags_ary.push(flag)
            }
            obj.insert("adds_flags", flags_ary);
        }
        if let Some(required_flags) = &self.required_flags {
            let mut flags_ary = Array::with_capacity(required_flags.len());
            for flag in required_flags {
                flags_ary.push(flag)
            }
            obj.insert("required_flags", flags_ary);
        }
        xff!(obj)
    }
    fn from_xff(value: &XffValue) -> SSCCResult<Modifier> {
        if let Some(obj) = value.as_object()
            && let Some(ibvp) = obj.get("impacted_base_value_pairs")
            && let Some(ibvp) = ibvp.as_array()
            && let Some(k) = obj.get("kind")
            && let Some(op) = obj.get("op")
        {
            let impacted_base_value_pairs = {
                let mut tmp_vec = Vec::with_capacity(ibvp.len());
                for bvp in ibvp {
                    tmp_vec.push(BaseValuePair::from_xff(bvp)?);
                }
                tmp_vec
            };
            let kind = ModifierType::from_xff(k)?;
            let op = ModifierOp::from_xff(op)?;
            let duration = obj.get("duration").and_then(|v| v.into_std_duration());
            let adds_flags = obj.get("adds_flags").and_then(|v| v.as_array()).map(|ary| {
                let mut tmp_vec = Vec::with_capacity(ary.len());
                for flag in ary {
                    if let Some(flag) = flag.into_string() {
                        tmp_vec.push(flag);
                    }
                }
                tmp_vec
            });
            let required_flags = obj
                .get("required_flags")
                .and_then(|v| v.as_array())
                .map(|ary| {
                    let mut tmp_vec = Vec::with_capacity(ary.len());
                    for flag in ary {
                        if let Some(flag) = flag.into_string() {
                            tmp_vec.push(flag);
                        }
                    }
                    tmp_vec
                });
            Ok(Modifier {
                impacted_base_value_pairs,
                kind,
                duration,
                adds_flags,
                required_flags,
                op,
            })
        } else {
            Err(new_xff_serde_error(
                "Can't convert value to Modifier, not an object.",
            ))
            .add_ctx(format!("Value: '{:?}'", value))
        }
    }
}
