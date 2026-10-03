use aequa::{Object, XffValue, hp_float::HpFloat, xff};

use crate::modules::{
    component::Component,
    modifiers::{Modifier, ModifierType},
};
#[derive(Debug, Clone)]
pub struct StatBox {
    /// Base values (no modifiers)
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^0/u32::MAX
    base_values: Object,
    /// Any modifier that modifies the base values
    /// Range: -1.0 - 1.0
    modifiers: Vec<Modifier>,
    /// Any modifier that is added to the base values before applying modifiers
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^u32::MAX
    flat_modifiers: Vec<Modifier>,
    /// Any modifier that does not currently modify the base values
    unused_modifiers: Vec<Modifier>,
    /// Any flat modifier that does not currently modify the base values
    unused_flat_modifiers: Vec<Modifier>,
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
    pub fn has_base_values(&self) -> bool {
        !self.base_values.is_empty()
    }
    pub fn get_modifiers(&self) -> &Vec<Modifier> {
        &self.modifiers
    }
    pub fn get_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.flat_modifiers
    }
    pub fn get_unused_modifiers(&self) -> &Vec<Modifier> {
        &self.unused_modifiers
    }
    pub fn get_unused_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.unused_flat_modifiers
    }
    /// Calculate and return all base values with their modifiers applied.
    ///
    /// Flat modifiers are applied before modifiers:
    /// `(base_value + flat_modifier) * modifier`
    ///
    /// Flat modifiers are calculated as:
    /// `flat_modifier1 + flat_modifier2 + flat_modifier3 + ...`
    ///
    /// Modifiers are calculated as:
    /// `1 + (modifier_value1 + modifier_value2 + modifier_value3 + ...)`
    ///
    /// These calculations are done when adding components or modifiers to the stat box.
    ///
    /// The result is a new object with the same keys as the base values
    ///
    /// # Example
    /// We assume:
    /// ```text
    /// base_values = {
    ///     "weight": 8.0,
    /// }
    /// flat_modifiers = {
    ///     "weight": -3.0,
    ///     "weight": -1.0,
    /// }
    /// modifiers = {
    ///     "weight": 0.5,
    ///     "weight": 0.5,
    /// }
    /// ```
    /// resulting in this formula:
    /// `(8.0 + (-3.0 + -1.0)) * (1 + (0.5 + 0.5))`
    /// = `4.0 * 2.0` = `8.0`
    ///
    /// Should any side of the multiplication be 0, the result will be 0.
    /// This is intended behavior.
    pub fn get_full_stats(&self) -> Object {
        let mut out = Object::new();
        for (key, value) in self.base_values.iter() {
            let flat_accu =
                self.flat_modifiers
                    .iter()
                    .fold(HpFloat::new_zero(), |accu, modifier| {
                        if &modifier.impacted_base_value_name == key {
                            accu + modifier.value
                        } else {
                            accu
                        }
                    });
            let mod_accu: HpFloat =
                self.modifiers
                    .iter()
                    .fold(HpFloat::new_zero(), |accu, modifier| {
                        if &modifier.impacted_base_value_name == key {
                            accu + modifier.value
                        } else {
                            accu
                        }
                    })
                    + 1.0;
            let mut base_with_flat = *value.as_hp_float().unwrap() + flat_accu;
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
        for (index, modifier) in self.unused_modifiers.iter().enumerate() {
            if self
                .base_values
                .contains_key(&modifier.impacted_base_value_name)
            {
                self.modifiers.push(modifier.clone());
                to_remove.push(index);
            }
        }
        for index in to_remove.iter().rev() {
            self.unused_modifiers.remove(*index);
        }
        to_remove.clear();
        for (index, modifier) in self.unused_flat_modifiers.iter().enumerate() {
            if self
                .base_values
                .contains_key(&modifier.impacted_base_value_name)
            {
                self.flat_modifiers.push(modifier.clone());
                to_remove.push(index);
            }
        }
        for index in to_remove.iter().rev() {
            self.unused_flat_modifiers.remove(*index);
        }
    }
    pub fn add_modifier(&mut self, modifier: Modifier) {
        let (modifiers, unused_modifiers) = match modifier.kind {
            ModifierType::Percentage => (&mut self.modifiers, &mut self.unused_modifiers),
            ModifierType::Flat => (&mut self.flat_modifiers, &mut self.unused_flat_modifiers),
        };
        if self
            .base_values
            .contains_key(&modifier.impacted_base_value_name)
        {
            modifiers.push(modifier);
        } else {
            unused_modifiers.push(modifier);
        }
    }
    pub fn add_component(&mut self, component: &Component) {
        let stat_box = component.get_stat_box();
        for (base_value_name, base_value_value) in stat_box.base_values.iter() {
            let base_value_value = base_value_value
                .as_hp_float()
                .expect("Base value is not a number");
            if let Some(held_base_value_value) = self.base_values.get_mut(base_value_name) {
                let held_base_value_value = held_base_value_value
                    .as_hp_float_mut()
                    .expect("Base value is not a number");
                *held_base_value_value = *held_base_value_value + *base_value_value;
            } else {
                self.base_values
                    .insert(base_value_name.clone(), *base_value_value);
            }
        }
        for modifier in stat_box.modifiers.iter() {
            self.add_modifier(modifier.clone());
        }
        for modifier in stat_box.flat_modifiers.iter() {
            self.add_modifier(modifier.clone());
        }
        for unused_modifier in stat_box.unused_modifiers.iter() {
            self.add_modifier(unused_modifier.clone());
        }
        for unused_flat_modifier in stat_box.unused_flat_modifiers.iter() {
            self.add_modifier(unused_flat_modifier.clone());
        }
        self.update_unused_modifiers();
    }
}

#[test]
fn simple_stat_box() {
    let mut stat_box = StatBox::new();
    stat_box.add_base_value("health", &HpFloat::from(10.0));
    let modifier = Modifier {
        impacted_base_value_name: "health".to_string(),
        kind: ModifierType::Percentage,
        value: 0.5.into(),
    };
    stat_box.add_modifier(modifier);
    let flat_modifier = Modifier {
        impacted_base_value_name: "health".to_string(),
        kind: ModifierType::Flat,
        value: 2.0.into(),
    };
    stat_box.add_modifier(flat_modifier);
    let result = stat_box.get_full_stats();
    assert!(result.len() == 1);
    assert!(result.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::from(18.0));
    let unused_modifier = Modifier {
        impacted_base_value_name: "weight".to_string(),
        kind: ModifierType::Percentage,
        value: HpFloat::from(-0.5),
    };
    stat_box.add_modifier(unused_modifier);
    let unused_flat_modifier = Modifier {
        impacted_base_value_name: "weight".to_string(),
        kind: ModifierType::Flat,
        value: HpFloat::from(-1.0),
    };
    stat_box.add_modifier(unused_flat_modifier);
    let result = stat_box.get_full_stats();
    assert!(result.len() == 1);
    stat_box.add_base_value("weight", &HpFloat::from(5.0));
    let result = stat_box.get_full_stats();
    assert!(result.len() == 2);

    assert!(result.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::from(18.0));
    assert!(result.get("weight").unwrap().as_hp_float().unwrap() == &HpFloat::from(2.0));
}
