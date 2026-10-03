use aequa::{Object, XffValue};

use crate::modules::{
    capabilites::{Capability, CapabilityKind},
    component::builder::ComponentBuilder,
    modifiers::Modifier,
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
    stat_box: StatBox,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Child components
    components: Vec<Component>,
}

impl Component {
    /// Create a new component.
    ///
    /// Initializes as a builder, call `.build()` when finished to validate the component.
    ///
    /// # Required Fields
    /// - `name` via [ComponentBuilder::add_name]
    /// - `description` via [ComponentBuilder::add_description]
    /// - At least one `base_value` via [ComponentBuilder::add_base_value]
    pub fn new() -> ComponentBuilder {
        ComponentBuilder::new(Component {
            name: String::new(),
            description: String::new(),
            stat_box: StatBox::new(),
            capabilities: Vec::new(),
            components: Vec::new(),
        })
    }
    /// Return the `StatBox` of the component.
    ///
    /// This contains all base values and modifiers.
    pub fn get_stat_box(&self) -> &StatBox {
        &self.stat_box
    }
    /// Return all modifiers of the component.
    ///
    /// These modifiers are percentage modifiers.
    ///
    /// In the general formula:
    /// `final_value = (base_value + sum(flat_modifiers)) * (1 + sum(modifiers))`
    /// They represent the `modifiers` part of the formula.
    ///
    /// Only 'in-use' modifiers are returned.
    ///
    /// Modifiers are held in the `StatBox`.
    pub fn get_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_modifiers()
    }
    /// Return all flat modifiers of the component.
    ///
    /// These modifiers are flat modifiers.
    ///
    /// In the general formula:
    /// `final_value = (base_value + sum(flat_modifiers)) * (1 + sum(modifiers))`
    /// They represent the `flat_modifiers` part of the formula.
    ///
    /// Only 'in-use' flat modifiers are returned.
    ///
    /// Flat modifiers are held in the `StatBox`.
    pub fn get_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_flat_modifiers()
    }
    /// Return all unused modifiers of the component.
    ///
    /// These modifiers are percentage modifiers.
    /// They are not used by any base value.
    ///
    /// Unused modifiers are held in the `StatBox`.
    pub fn get_unused_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_unused_modifiers()
    }
    /// Return all unused flat modifiers of the component.
    ///
    /// These modifiers are flat modifiers.
    /// They are not used by any base value.
    ///
    /// Unused flat modifiers are held in the `StatBox`.
    pub fn get_unused_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.stat_box.get_unused_flat_modifiers()
    }
    /// Return all child components
    pub fn get_components(&self) -> &Vec<Component> {
        &self.components
    }
    /// Return all capabilities of the component
    pub fn get_capabilities(&self) -> &Vec<Capability> {
        &self.capabilities
    }
    /// Check if the component has a specific capability
    pub fn has_capacity_for(&self, capabilitykind: CapabilityKind) -> bool {
        self.capabilities.iter().any(|c| c.is_kind(&capabilitykind))
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
        let value = value.into_hp_float().unwrap();
        self.stat_box.add_base_value(key, &value);
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
    /// Add a capability
    ///
    /// Any base value and modifiers from the capability are added to the current component.
    pub fn add_capability(&mut self, capability: Capability) {
        if let Some(base_value_name) = capability.base_value_name()
            && let Some(base_value_value) = capability.base_value_value()
        {
            self.stat_box
                .add_or_update_base_value(base_value_name, base_value_value);
        }
        for modifier in capability.inherent_modifiers() {
            self.add_modifier(modifier.clone());
        }
        self.capabilities.push(capability);
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

mod tests {
    use aequa::{XffValue, hp_float::HpFloat, xff};

    use crate::modules::{
        capabilites::{Capability, CapabilityKind},
        component::Component,
        modifiers::{Modifier, ModifierType},
    };

    fn make_test_component() -> Component {
        Component::new()
            .add_name("Test Component")
            .add_description("Test")
            .add_base_value("health", xff!(2.0))
            .build()
            .unwrap()
    }

    #[test]
    fn basic_component_builder() {
        let component = Component::new()
            .add_name("Test Component")
            .add_description("Test")
            .add_base_value("health", xff!(2.0))
            .build();
        assert!(component.is_ok());
    }

    #[test]
    fn full_component() {
        let mut component = make_test_component();
        let modifier = Modifier {
            impacted_base_value_name: "health".to_string(),
            kind: ModifierType::Percentage,
            value: 0.5.into(),
        };
        let flat_modifier = Modifier {
            impacted_base_value_name: "health".to_string(),
            kind: ModifierType::Flat,
            value: 2.0.into(),
        };
        component.add_modifier(modifier.clone());
        component.add_modifier(flat_modifier);
        let stats = component.get_full_stats();
        assert!(stats.contains_key("health"));
        assert!(stats.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(6, 0));
        component.add_component(make_test_component());
        let stats = component.get_full_stats();
        assert!(stats.contains_key("health"));
        assert!(stats.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(9, 0));
        let capability = Capability::new()
            .add_name("Test Capability")
            .add_description("Test")
            .add_kind(CapabilityKind::SpaceFlight)
            .add_inherent_modifier(modifier)
            .build();
        assert!(capability.is_ok());
        let capability = capability.unwrap();
        component.add_capability(capability);
        let stats = component.get_full_stats();
        assert!(stats.contains_key("health"));
        assert!(stats.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(12, 0));
    }

    #[test]
    fn basic_component_stats() {
        let mut component = make_test_component();
        let stats = component.get_full_stats();
        assert!(stats.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(2, 0));

        let modifier = Modifier {
            impacted_base_value_name: "health".to_string(),
            kind: ModifierType::Percentage,
            value: 0.5.into(),
        };
        component.add_modifier(modifier);
        let stats = component.get_full_stats();
        assert!(stats.contains_key("health"));
        assert!(stats.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(3, 0));
    }

    #[test]
    fn builder_validation() {
        let component = Component::new()
            .add_description("Test")
            .add_base_value("health", xff!(1.0))
            .build();
        assert!(component.is_err());
        let component = Component::new()
            .add_name("Test Component")
            .add_base_value("health", xff!(1.0))
            .build();
        assert!(component.is_err());
        let component = Component::new()
            .add_name("Test Component")
            .add_description("Test")
            .build();
        assert!(component.is_err());
    }
}
