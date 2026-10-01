use aequa::{Object, XffValue};

use crate::modules::{Capability, component::builder::ComponentBuilder};

mod builder;

/// A single component.
///
/// May be made up of other components
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Component {
    /// Name of the component
    name: String,
    /// Description
    description: String,
    /// Base values (no modifiers)
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^u32::MAX
    base_values: Object,
    /// Any modifier that modifies the base values
    /// Range: -1.0 - 1.0
    modifiers: Object,
    /// Any modifier that is added to the base values before applying modifiers
    /// Range: -inf - inf
    /// Technically (HpFloat): i128::MAX/MIN * 10^u32::MAX
    flat_modifiers: Object,
    /// Any modifier that does not currently modify the base values
    unused_modifiers: Object,
    /// Any flat modifier that does not currently modify the base values
    unused_flat_modifiers: Object,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Child components
    components: Vec<Component>,
}

impl Component {
    pub fn new() -> ComponentBuilder {
        ComponentBuilder::new()
    }
    pub fn get_modifiers(&self) -> &Object {
        &self.modifiers
    }
    pub fn get_unused_modifiers(&self) -> &Object {
        &self.unused_modifiers
    }
    pub fn get_components(&self) -> &Vec<Component> {
        &self.components
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
    pub fn get_full_stats(&self) -> Object {
        // NOTE: Take extra care for negatvie base_value and negative calculated modifiers.
        // Should a value (e.g weight) be 2.0 and the flat modifier -2.0 and the modifiers to lower
        // the weight are -0.5 and -0.5:
        //
        // With the doc comment formula:
        // (2.0 + (-2.0)) * (1 + (-0.5) + (-0.5))
        // = 0.0 * (1 - 1.0)
        // = 0.0 * 0.0
        // = 0.0 -> Would be still fine-ish if unrealistic
        //
        // flat: -3.0; modifiers: -0.75, -0.75
        // (2.0 + (-3.0)) * (1 + (-0.75) + (-0.75))
        // = -1.0 * (1 - 1.5)
        // = -1.0 * -0.5
        // = 0.5 -> Now its increasing. Not fine
        //
        // As long as both sides have different signs, or both are positive, everything
        // works.
        //
        // Should both be negative, maybe just `* -1.0` in the end?
        todo!()
    }
    /// Add a base value
    ///
    /// If a base value with the key exists, it will be overwritten
    ///
    /// Does not update unused modifiers, use `update_unused_modifiers` after calling this.
    pub fn add_base_value<S: Into<String>>(&mut self, key: S, value: XffValue) {
        self.base_values.insert(key, value);
    }
    /// Iterate through all unused modifiers and check if a base value exists they can modify.
    /// If so they are moved to modifiers / flat modifiers.
    ///
    /// Checks both `modifiers` and `flat_modifiers`
    pub fn update_unused_modifiers(&mut self) {
        for (modifier_name, modifier) in self.unused_modifiers.iter() {
            if self.base_values.contains_key(modifier_name) {
                self.modifiers
                    .insert(modifier_name.clone(), modifier.clone());
            }
        }
        for (modifier_name, modifier) in self.unused_flat_modifiers.iter() {
            if self.base_values.contains_key(modifier_name) {
                self.flat_modifiers
                    .insert(modifier_name.clone(), modifier.clone());
            }
        }
    }
    pub fn add_modifier(&mut self, modifier: Object) {
        todo!("Same code as in builder")
    }
    pub fn add_flat_modifier(&mut self, modifier: Object) {
        todo!("Same code as in builder")
    }
    pub fn add_component(&mut self, component: Component) {
        todo!("Same code as in builder")
    }
}
