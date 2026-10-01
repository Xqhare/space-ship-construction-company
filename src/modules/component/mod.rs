use aequa::{Object, XffValue};

use crate::modules::{
    Capability,
    component::builder::ComponentBuilder,
    modifiers::{Modifier, ModifierType},
    stat_box::StatBox,
};

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
    /// Statistics
    pub stat_box: StatBox,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Child components
    components: Vec<Component>,
}

impl Component {
    pub fn new() -> ComponentBuilder {
        ComponentBuilder::new(Component {
            name: String::new(),
            description: String::new(),
            stat_box: StatBox::new(),
            capabilities: Vec::new(),
            components: Vec::new(),
        })
    }
    pub fn get_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_modifiers()
    }
    pub fn get_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_flat_modifiers()
    }
    pub fn get_unused_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_unused_modifiers()
    }
    pub fn get_unused_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_unused_flat_modifiers()
    }
    pub fn get_components(&self) -> &Vec<Component> {
        &self.components
    }
    pub fn get_capabilities(&self) -> &Vec<Capability> {
        &self.capabilities
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
        self.stat_box.get_full_stats()
    }
    /// Add a base value
    ///
    /// If a base value with the key exists, it will be overwritten
    ///
    /// Does not update unused modifiers, use `update_unused_modifiers` after calling this.
    pub fn add_base_value<S: Into<String>>(&mut self, key: S, value: XffValue) {
        self.stat_box.add_base_value(key, value);
    }
    /// Iterate through all unused modifiers and check if a base value exists they can modify.
    /// If so they are moved to modifiers / flat modifiers.
    ///
    /// Checks both `modifiers` and `flat_modifiers`
    pub fn update_unused_modifiers(&mut self) {
        self.stat_box.update_unused_modifiers();
    }
    pub fn add_modifier(&mut self, modifier: Modifier) {
        self.stat_box.add_modifier(modifier);
    }
    /// Add a component.
    ///
    /// Base values and modifiers from the component are added to the current component
    /// directly.
    ///
    /// Base value is additive. Multiple components with the same modifier will stack
    pub fn add_component(&mut self, component: Component) {
        self.capabilities.extend(component.capabilities.clone());
        self.stat_box.add_component(&component);
        self.components.push(component);
    }
}
