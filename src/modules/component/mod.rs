use aequa::{Object, XffValue};

use crate::{
    modules::{
        component::{builder::ComponentBuilder, capability::Capability},
        modifier::Modifier,
    },
    utils::boxes::data_box::{DataBox, flag_box::Flag},
};

mod builder;
pub mod capability;

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
    data_box: DataBox,
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
            data_box: DataBox::new(),
            capabilities: Vec::new(),
            components: Vec::new(),
        })
    }
    /// Return the `dataBox` of the component.
    ///
    /// This contains all base values and modifiers.
    pub fn get_data_box(&self) -> &DataBox {
        &self.data_box
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
    /// Modifiers are held in the `dataBox`.
    pub fn get_modifiers(&self) -> &Vec<Modifier> {
        &self.data_box.get_modifiers()
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
    /// Flat modifiers are held in the `dataBox`.
    pub fn get_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.data_box.get_flat_modifiers()
    }
    /// Return all unused modifiers of the component.
    ///
    /// These modifiers are percentage modifiers.
    /// They are not used by any base value.
    ///
    /// Unused modifiers are held in the `dataBox`.
    pub fn get_unused_modifiers(&self) -> &Vec<Modifier> {
        &self.data_box.get_unused_modifiers()
    }
    /// Return all unused flat modifiers of the component.
    ///
    /// These modifiers are flat modifiers.
    /// They are not used by any base value.
    ///
    /// Unused flat modifiers are held in the `dataBox`.
    pub fn get_unused_flat_modifiers(&self) -> &Vec<Modifier> {
        &self.data_box.get_unused_flat_modifiers()
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
    pub fn has_capacity_flag(&self, capability_flag: &Flag) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.has_flag(capability_flag))
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
    pub fn get_full_data(&self) -> Object {
        self.data_box.
    }
    /// Add a base value
    ///
    /// If a base value with the key exists, it will be overwritten
    ///
    /// Does not update unused modifiers, use `update_unused_modifiers` after calling this.
    pub fn add_base_value<S: Into<String>>(&mut self, key: S, value: XffValue) {
        let value = value.into_hp_float().unwrap();
        self.data_box.add_base_value(key, &value);
    }
    /// Iterate through all unused modifiers and check if a base value exists they can modify.
    /// If so they are moved to modifiers / flat modifiers.
    ///
    /// Checks both `modifiers` and `flat_modifiers`
    pub fn update_unused_modifiers(&mut self) {
        self.data_box.update_unused_modifiers();
    }
    pub fn add_modifier(&mut self, modifier: Modifier) {
        self.data_box.add_modifier(modifier);
    }
    /// Add a capability
    ///
    /// Any base value and modifiers from the capability are added to the current component.
    pub fn add_capability(&mut self, capability: Capability) {
        if let Some(base_value_pair) = capability.impacted_base_value_pair() {
            let (base_value_name, base_value_value) = base_value_pair.as_parts();
            self.data_box
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
        for capability in component.get_capabilities() {
            self.add_capability(capability.clone());
        }
        self.data_box.add_component(&component);
        self.components.push(component);
    }
}

mod tests {
    use aequa::{XffValue, hp_float::HpFloat, xff};

    use crate::{
        consts::flags::capabilities::SPACE_FLIGHT,
        modules::{
            component::{Component, capability::Capability},
            modifier::{Modifier, op::ModifierOp, r#type::ModifierType},
        },
        needs_home::BaseValuePair,
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
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(10, 0))],
            kind: ModifierType::Percentage,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Add,
        };
        let flat_modifier = Modifier {
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(2, 0))],
            kind: ModifierType::Flat,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Add,
        };
        component.add_modifier(modifier.clone());
        component.add_modifier(flat_modifier);
        let datas = component.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(6, 0));
        component.add_component(make_test_component());
        let datas = component.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(9, 0));
        let capability = Capability::new()
            .add_name("Test Capability")
            .add_description("Test")
            .add_flag(SPACE_FLIGHT.to_string())
            .add_inherent_modifier(modifier)
            .build();
        assert!(capability.is_ok());
        let capability = capability.unwrap();
        component.add_capability(capability);
        let datas = component.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(12, 0));
    }

    #[test]
    fn basic_component_datas() {
        let mut component = make_test_component();
        let datas = component.get_full_stats();
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(2, 0));

        let modifier = Modifier {
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(5, 1))],
            kind: ModifierType::Percentage,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Mul,
        };
        component.add_modifier(modifier);
        let datas = component.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(3, 0));
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
