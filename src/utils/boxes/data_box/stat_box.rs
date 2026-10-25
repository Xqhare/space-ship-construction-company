use std::time::Duration;

use aequa::{Array, Object, XffSerde, XffValue, hp_float::HpFloat, xff};
use horae::Utc;

use crate::{
    error::{SSCCResult, new_startup_error},
    modules::{
        component::Component,
        modifier::{Modifier, ModifierWithEndUtc, op::ModifierOp, r#type::ModifierType},
    },
    needs_home::BaseValuePair,
};

#[derive(Debug, Clone)]
pub struct StatBox {
    /// Base values (no modifiers)
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^0/u32::MAX
    base_values: Object,
    /// Any modifier that modifies the base values
    /// Range: -1.0 - 1.0
    modifiers: Vec<ModifierWithEndUtc>,
    /// Any modifier that is added to the base values before applying modifiers
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^u32::MAX
    flat_modifiers: Vec<ModifierWithEndUtc>,
    /// Any modifier that does not currently modify the base values
    unused_modifiers: Vec<ModifierWithEndUtc>,
    /// Any flat modifier that does not currently modify the base values
    unused_flat_modifiers: Vec<ModifierWithEndUtc>,
}

impl StatBox {
    pub fn new() -> StatBox {
        StatBox {
            base_values: Object::new(),
            modifiers: Vec::new(),
            flat_modifiers: Vec::new(),
            unused_modifiers: Vec::new(),
            unused_flat_modifiers: Vec::new(),
        }
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("base_values", self.base_values.clone());
        let modifiers: Array = self.modifiers.iter().map(|m| m.to_xff()).collect();
        obj.insert("modifiers", modifiers);
        let flat_modifiers: Array = self.flat_modifiers.iter().map(|m| m.to_xff()).collect();
        obj.insert("flat_modifiers", flat_modifiers);
        let unused_modifiers: Array = self.unused_modifiers.iter().map(|m| m.to_xff()).collect();
        obj.insert("unused_modifiers", unused_modifiers);
        let unused_flat_modifiers: Array = self
            .unused_flat_modifiers
            .iter()
            .map(|m| m.to_xff())
            .collect();
        obj.insert("unused_flat_modifiers", unused_flat_modifiers);
        xff!(obj)
    }
    pub fn from_xff(xff: &XffValue) -> SSCCResult<StatBox> {
        if let Some(obj) = xff.as_object()
            && let Some(base_values) = obj.get("base_values")
            && let Some(base_values) = base_values.as_object()
            && let Some(modifiers) = obj.get("modifiers")
            && let Some(modifiers) = modifiers.as_array()
            && let Some(flat_modifiers) = obj.get("flat_modifiers")
            && let Some(flat_modifiers) = flat_modifiers.as_array()
            && let Some(unused_modifiers) = obj.get("unused_modifiers")
            && let Some(unused_modifiers) = unused_modifiers.as_array()
            && let Some(unused_flat_modifiers) = obj.get("unused_flat_modifiers")
            && let Some(unused_flat_modifiers) = unused_flat_modifiers.as_array()
        {
            let modifiers = {
                let mut out = Vec::new();
                for modifier in modifiers {
                    out.push(ModifierWithEndUtc::from_xff(&modifier)?);
                }
                out
            };
            let flat_modifiers = {
                let mut out = Vec::new();
                for modifier in flat_modifiers {
                    out.push(ModifierWithEndUtc::from_xff(&modifier)?);
                }
                out
            };
            let unused_modifiers = {
                let mut out = Vec::new();
                for modifier in unused_modifiers {
                    out.push(ModifierWithEndUtc::from_xff(&modifier)?);
                }
                out
            };
            let unused_flat_modifiers = {
                let mut out = Vec::new();
                for modifier in unused_flat_modifiers {
                    out.push(ModifierWithEndUtc::from_xff(&modifier)?);
                }
                out
            };
            Ok(StatBox {
                base_values: base_values.clone(),
                modifiers,
                flat_modifiers,
                unused_modifiers,
                unused_flat_modifiers,
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to StatBox, not an object or doesnt have expected keys.",
            )
            .add_ctx(format!("Value: {:?}", xff)))
        }
    }
    /// Checks if the passed arg is tracked as a base value.
    pub fn is_tracking<S: Into<String>>(&self, base_value_name: S) -> bool {
        self.base_values.contains_key(&base_value_name.into())
    }
    pub fn has_base_values(&self) -> bool {
        !self.base_values.is_empty()
    }
    pub fn get_modifiers(&self) -> &Vec<ModifierWithEndUtc> {
        &self.modifiers
    }
    pub fn get_flat_modifiers(&self) -> &Vec<ModifierWithEndUtc> {
        &self.flat_modifiers
    }
    pub fn get_unused_modifiers(&self) -> &Vec<ModifierWithEndUtc> {
        &self.unused_modifiers
    }
    pub fn get_unused_flat_modifiers(&self) -> &Vec<ModifierWithEndUtc> {
        &self.unused_flat_modifiers
    }
    /// Calculates and returns all base values with their flat and percentage modifiers applied.
    ///
    /// # Formula
    ///
    /// For each stat in `base_values`, the final value is computed in three stages:
    ///
    /// 1. **Flat Accumulation**: Starts at `0.0` and applies all matching `flat_modifiers`.
    /// 2. **Percentage Accumulation**: Starts at `1.0` (100%) and applies all matching `modifiers`.
    /// 3. **Final Combination**:
    ///    ```text
    ///    base_with_flat = base_value + flat_accumulated
    ///    final_value    = base_with_flat * percentage_accumulated
    ///    ```
    ///
    /// ### Sign Preservation (Double-Negative Guard)
    /// If both `base_with_flat` and `percentage_accumulated` are negative, `base_with_flat` is
    /// negated before multiplication. This prevents two negative values from canceling each other
    /// out into a positive stat, ensuring the final result remains negative.
    ///
    /// # Modifier Evaluation Order
    ///
    /// Both the flat and percentage accumulators evaluate their modifier lists using the same rules:
    ///
    /// 1. **Sequential Arithmetic** (evaluated in order of appearance):
    ///    * [`ModifierOp::Add`] / [`ModifierOp::Sub`]: Adds to or subtracts from the accumulator.
    ///    * [`ModifierOp::Mul`] / [`ModifierOp::Div`]: Multiplies or divides the current accumulator.
    ///      *(Note: Should the accumulator be 0.0 and any multiplication is attempted, the resultant value will be the modifier value).*
    ///      *(Note: a division by `0.0` will be ignored and not do anything)*
    /// 2. **Upper Bound ([`ModifierOp::Lim`])**: Applied after all arithmetic operations. Caps the
    ///    accumulator at the specified maximum. If multiple `Lim` modifiers exist for the same stat,
    ///    the **highest** limit takes precedence.
    /// 3. **Absolute Override ([`ModifierOp::Set`])**: Applied last, overriding both arithmetic results
    ///    and any `Lim` caps. If multiple `Set` modifiers exist, the **highest** value takes precedence.
    ///
    /// # Panics
    ///
    /// Panics if any value in `self.base_values` cannot be converted to an [`HpFloat`]
    pub fn get_full_stats(&self) -> Object {
        let mut out = Object::new();
        for (key, value) in self.base_values.iter() {
            let flat_accu = make_mod_accum(HpFloat::new_zero(), &self.flat_modifiers, key);
            let mod_accu = make_mod_accum(HpFloat::new(1, 0), &self.modifiers, key);
            let mut base_with_flat = *value
                .as_hp_float()
                .expect("Base value must only consist of Number values")
                + flat_accu;
            if base_with_flat.is_negative() && mod_accu.is_negative() {
                base_with_flat = base_with_flat * -1.0;
            }
            let final_value = base_with_flat * mod_accu;
            out.insert(key, final_value);
        }
        out
    }
    /// Add a base value
    ///
    /// If a base value with the key exists, it will be overwritten
    ///
    /// Does not update unused modifiers, use `update_unused_modifiers` after calling this.
    pub fn add_base_value<S: Into<String>>(&mut self, key: S, value: &HpFloat) {
        self.base_values.insert(key, xff!(*value));
        self.update_unused_modifiers();
    }
    /// Add or update a base value.
    /// If a base value with the key exists, it's value and the new value will be added
    /// together for a final value.
    /// If a base value with the key does not exist, it will be added.
    pub fn add_or_update_base_value<S: Into<String>>(&mut self, key: S, value: &HpFloat) {
        let key = key.into();
        if let Some(old_value) = self.base_values.get_mut(&key) {
            if let Some(old_value_cast) = old_value.into_hp_float() {
                *old_value = xff!(old_value_cast + *value);
            }
        } else {
            self.base_values.insert(key, xff!(*value));
        }
        self.update_unused_modifiers();
    }
    /// Iterate through all unused modifiers and check if a base value exists they can modify.
    /// If so they are moved to modifiers / flat modifiers.
    ///
    /// Checks both `modifiers` and `flat_modifiers`
    pub fn update_unused_modifiers(&mut self) {
        let mut to_remove = Vec::new();
        for (index, modifier_mod) in self.unused_modifiers.iter().enumerate() {
            let modifier = &modifier_mod.modifier;
            for pair in &modifier.impacted_base_value_pairs {
                if self.base_values.contains_key(&pair.base_value_name) {
                    self.modifiers.push(modifier_mod.clone());
                    to_remove.push(index);
                    break;
                }
            }
        }
        for index in to_remove.iter().rev() {
            self.unused_modifiers.remove(*index);
        }
        to_remove.clear();
        for (index, modifier_mod) in self.unused_flat_modifiers.iter().enumerate() {
            let modifier = &modifier_mod.modifier;
            for pair in &modifier.impacted_base_value_pairs {
                if self.base_values.contains_key(&pair.base_value_name) {
                    self.flat_modifiers.push(modifier_mod.clone());
                    to_remove.push(index);
                    break;
                }
            }
        }
        for index in to_remove.iter().rev() {
            self.unused_flat_modifiers.remove(*index);
        }
    }
    /// Drop all modifiers (used or not) that are past their end date
    pub fn update_time_limited_modifiers(&mut self, now_utc: Utc) {
        self.modifiers = drop_modifiers_past_date(&self.modifiers, now_utc);
        self.unused_modifiers = drop_modifiers_past_date(&self.unused_modifiers, now_utc);
        self.flat_modifiers = drop_modifiers_past_date(&self.flat_modifiers, now_utc);
        self.unused_flat_modifiers = drop_modifiers_past_date(&self.unused_flat_modifiers, now_utc);
    }
    /// Update all used and unused modifiers as well as dropping all modifiers that are past their end date
    pub fn update_all(&mut self, now_utc: Utc) {
        self.update_time_limited_modifiers(now_utc);
        self.update_unused_modifiers();
    }
    /// Add a modifier to the component.
    ///
    /// Will be added as an unused modifier if no base value the modifier modifies exists in
    /// the box
    ///
    /// # Arguments
    /// * `modifier`: Modifier to add as a `ModifierWithEndUtc`
    pub fn add_modifier(&mut self, modifier: ModifierWithEndUtc) {
        let modifier_mod = &modifier.modifier;
        let (modifiers, unused_modifiers) = match modifier_mod.kind {
            ModifierType::Percentage => (&mut self.modifiers, &mut self.unused_modifiers),
            ModifierType::Flat => (&mut self.flat_modifiers, &mut self.unused_flat_modifiers),
        };
        let mut was_contained = false;
        for pair in &modifier_mod.impacted_base_value_pairs {
            let (name, _) = pair.as_parts();

            if self.base_values.contains_key(name) {
                modifiers.push(modifier.clone());
                was_contained = true;
                break;
            }
        }
        if !was_contained {
            unused_modifiers.push(modifier);
        }
    }
    /// Add multiple modifiers to the component
    pub fn add_modifiers(&mut self, modifiers: Vec<ModifierWithEndUtc>) {
        for modifier in modifiers {
            self.add_modifier(modifier);
        }
    }
}

/// Accumulates all modifiers matching `base_value_name` starting from `start_val`.
///
/// # Arguments
///
/// * `start_val` - The initial accumulator value (`0.0` for flat modifiers or `1.0` for multipliers).
/// * `modifiers` - The slice of active modifiers to filter and evaluate.
/// * `base_value_name` - The stat key used to filter relevant modifier pairs.
///
/// # Evaluation Precedence
///
/// 1. Arithmetic operations (`Add`, `Sub`, `Mul`, `Div`) are applied sequentially to `start_val`.
/// 2. If one or more `Lim` operations are present, the accumulator is clamped to the maximum `Lim` value.
/// 3. If one or more `Set` operations are present, the accumulator is overwritten by the maximum `Set` value,
///    ignoring prior arithmetic and `Lim` constraints.
fn make_mod_accum(
    start_val: HpFloat,
    modifiers: &[ModifierWithEndUtc],
    base_value_name: &str,
) -> HpFloat {
    let mut accum = start_val;
    let mut limit = None;
    let mut set = None;
    for modifier in modifiers.iter() {
        let modifier = &modifier.modifier;
        for pair in modifier.get_pairs(base_value_name) {
            let value = pair.base_value_value;
            match modifier.op {
                ModifierOp::Add => accum = accum + value,
                ModifierOp::Sub => accum = accum - value,
                ModifierOp::Mul => {
                    if !accum.is_zero() {
                        accum = accum * value;
                    } else {
                        accum = value;
                    }
                }
                ModifierOp::Div => {
                    if !value.is_zero() {
                        accum = accum / value;
                    }
                }
                ModifierOp::Set => {
                    if set.is_none() || set < (Some(value)) {
                        set = Some(value);
                    }
                }
                ModifierOp::Lim => {
                    if limit.is_none() || limit < (Some(value)) {
                        limit = Some(value);
                    }
                }
            }
        }
    }
    if let Some(limit) = limit
        && accum > limit
    {
        accum = limit;
    }
    if let Some(set) = set {
        accum = set;
    }
    accum
}

fn drop_modifiers_past_date(
    modifiers: &[ModifierWithEndUtc],
    now_utc: Utc,
) -> Vec<ModifierWithEndUtc> {
    let mut out = Vec::with_capacity(modifiers.len());
    for modifier in modifiers {
        if modifier.end_utc > Some(now_utc) {
            out.push(modifier.clone());
        }
    }
    out
}

#[test]
fn simple_stat_box() {
    let mut stat_box = StatBox::new();
    let utc = Utc::now();
    stat_box.add_base_value("health", &HpFloat::from(10.0));
    let modifier = Modifier {
        impacted_base_value_pairs: vec![BaseValuePair {
            base_value_name: "health".to_string(),
            base_value_value: HpFloat::from(2.0),
        }],
        kind: ModifierType::Percentage,
        duration: None,
        adds_flags: None,
        required_flags: None,
        op: ModifierOp::Add,
    };
    stat_box.add_modifier(ModifierWithEndUtc::new(modifier, utc));
    let flat_modifier = Modifier {
        impacted_base_value_pairs: vec![BaseValuePair {
            base_value_name: "health".to_string(),
            base_value_value: HpFloat::from(2.0),
        }],
        kind: ModifierType::Flat,
        duration: None,
        adds_flags: None,
        required_flags: None,
        op: ModifierOp::Add,
    };
    stat_box.add_modifier(ModifierWithEndUtc::new(flat_modifier, utc));
    let result = stat_box.get_full_stats();
    assert!(result.len() == 1);
    assert!(result.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::from(18.0));
    let unused_modifier = Modifier {
        impacted_base_value_pairs: vec![BaseValuePair {
            base_value_name: "weight".to_string(),
            base_value_value: HpFloat::from(2.0),
        }],
        kind: ModifierType::Percentage,
        duration: None,
        adds_flags: None,
        required_flags: None,
        op: ModifierOp::Add,
    };
    stat_box.add_modifier(ModifierWithEndUtc::new(unused_modifier, utc));
    let unused_flat_modifier = Modifier {
        impacted_base_value_pairs: vec![BaseValuePair {
            base_value_name: "weight".to_string(),
            base_value_value: HpFloat::from(2.0),
        }],
        kind: ModifierType::Flat,
        duration: None,
        adds_flags: None,
        required_flags: None,
        op: ModifierOp::Add,
    };
    stat_box.add_modifier(ModifierWithEndUtc::new(unused_flat_modifier, utc));
    let result = stat_box.get_full_stats();
    assert!(result.len() == 1);
    stat_box.add_base_value("weight", &HpFloat::from(5.0));
    let result = stat_box.get_full_stats();
    assert!(result.len() == 2);

    assert!(result.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::from(18.0));
    assert!(result.get("weight").unwrap().as_hp_float().unwrap() == &HpFloat::from(2.0));
}
