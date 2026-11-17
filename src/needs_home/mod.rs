use std::collections::HashSet;

use aequa::{Object, XffSerde, XffValue, xff};
use athena::float::HpFloat;
use nemesis::NemesisError;

use crate::{
    consts::{characters::traits::CharacterTrait, normalise_bvp_name},
    error::new_xff_serde_error,
    modules::modifier::r#type::ModifierType,
    utils::op::Op,
};

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

/// Accumulates all [`TraitStatImpact`](crate::consts::characters::traits::impact::TraitStatImpact)s
/// across `traits` into `(base_value_name, (flat_accum, perc_accum))` pairs.
///
/// # Evaluation Rules
/// 1. **Group Key Expansion**: Each impact's `base_value_name` is expanded via
///    [`normalise_bvp_name`] (e.g. `SCI_BONUS_ALL_FIELDS` expands to all science field names, and
///    `SOC_BONUS_ALL` expands to `sociability`).
/// 2. **Sequential Arithmetic**: For each affected base value, `flat_accum` starts at `0.0` and
///    `perc_accum` starts at `1.0` (`100%`). Impacts are applied in slice order using [`Op::Add`],
///    [`Op::Sub`], [`Op::Mul`] (if the accumulator is `0.0`, multiplication sets it to `value`),
///    and [`Op::Div`] (division by `0.0` is ignored).
/// 3. **Limits & Overrides**: After arithmetic operations, [`Op::Lim`] clamps the accumulator to
///    the highest limit value if exceeded, and [`Op::Set`] overrides the accumulator with the
///    highest set value.
/// 4. **Application**: [`Character::get_full_stats`](crate::modules::character::Character::get_full_stats)
///    applies each returned tuple as `*stat = (*stat + flat_accum) * perc_accum` for stats tracked
///    in the character's `StatBox`.
pub fn accum_char_traits(
    traits: &[&'static CharacterTrait],
) -> Vec<(&'static str, (HpFloat, HpFloat))> {
    let all_impacted_bv_names = traits
        .iter()
        .flat_map(|entry| {
            entry
                .affected_base_value_pairs()
                .iter()
                .map(|element| normalise_bvp_name(element.base_value_name))
        })
        .flatten()
        .collect::<HashSet<&'static str>>();
    let mut out = Vec::with_capacity(all_impacted_bv_names.len());
    for impacted_bv_name in all_impacted_bv_names {
        let mut flat_accum = HpFloat::new_zero();
        let mut perc_accum = HpFloat::new(1, 0);
        let mut flat_limit = None;
        let mut perc_limit = None;
        let mut flat_set = None;
        let mut perc_set = None;
        for tr in traits {
            for impact in tr.affected_base_value_pairs() {
                if normalise_bvp_name(impact.base_value_name).contains(&impacted_bv_name) {
                    let (accum, limit, set) = match impact.kind {
                        ModifierType::Flat => (&mut flat_accum, &mut flat_limit, &mut flat_set),
                        ModifierType::Percentage => {
                            (&mut perc_accum, &mut perc_limit, &mut perc_set)
                        }
                    };
                    let value: HpFloat = impact.base_value_value.into();
                    match impact.op {
                        Op::Add => *accum = *accum + value,
                        Op::Sub => *accum = *accum - value,
                        Op::Mul => {
                            if !accum.is_zero() {
                                *accum = *accum * value;
                            } else {
                                *accum = value;
                            }
                        }
                        Op::Div => {
                            if !value.is_zero() {
                                *accum = *accum / value;
                            }
                        }
                        Op::Set => {
                            if set.is_none() || *set < (Some(value)) {
                                *set = Some(value);
                            }
                        }
                        Op::Lim => {
                            if limit.is_none() || *limit < (Some(value)) {
                                *limit = Some(value);
                            }
                        }
                    }
                }
            }
        }
        if let Some(flat_limit) = flat_limit
            && flat_accum > flat_limit
        {
            flat_accum = flat_limit;
        }
        if let Some(perc_limit) = perc_limit
            && perc_accum > perc_limit
        {
            perc_accum = perc_limit;
        }
        if let Some(flat_set) = flat_set {
            flat_accum = flat_set;
        }
        if let Some(perc_set) = perc_set {
            perc_accum = perc_set;
        }
        out.push((impacted_bv_name, (flat_accum, perc_accum)));
    }
    out
}
