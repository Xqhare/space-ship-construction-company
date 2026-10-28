use crate::{
    modules::component::{builder::ComponentBuilder, capability::Capability},
    runtime::DataBoxSlotId,
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
    data_box_id: DataBoxSlotId,
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
    pub fn new(data_box_id: DataBoxSlotId) -> ComponentBuilder {
        ComponentBuilder::new(Component {
            name: String::new(),
            description: String::new(),
            data_box_id,
            capabilities: Vec::new(),
            components: Vec::new(),
        })
    }
    /// Return the `dataBox` of the component.
    ///
    /// This contains all base values and modifiers.
    pub fn get_data_box(&self) -> &DataBoxSlotId {
        &self.data_box_id
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
    /// Add a capability
    ///
    /// Any base value and modifiers from the capability are added to the current component.
    pub fn add_capability(&mut self, capability: Capability, data_box_ref: &mut DataBox) {
        if let Some(base_value_pair) = capability.impacted_base_value_pair() {
            let (base_value_name, base_value_value) = base_value_pair.as_parts();
            data_box_ref
                .get_stat_box_mut()
                .add_or_update_base_value(base_value_name, base_value_value);
        }
        for modifier in capability.inherent_modifiers() {
            data_box_ref
                .get_stat_box_mut()
                .add_modifier(modifier.clone());
        }
        self.capabilities.push(capability);
    }
    /// Add a component.
    ///
    /// Base values and modifiers from the component are added to the current component
    /// directly.
    ///
    /// Base value is additive. Multiple components with the same modifier will stack
    pub fn add_component(&mut self, component: Component, data_box_ref: &mut DataBox) {
        for capability in component.get_capabilities() {
            self.add_capability(capability.clone(), data_box_ref);
        }
        for child_comp in component.get_components() {
            self.add_component(child_comp.clone(), data_box_ref);
        }
        self.components.push(component);
    }
}

#[cfg(test)]
mod tests {
    use aequa::hp_float::HpFloat;
    use athena::slot_map::{SlotMap, SlotMapIndex};

    use crate::{
        consts::flags::capabilities::SPACE_FLIGHT,
        modules::{
            component::{Component, capability::Capability},
            modifier::{Modifier, ModifierWithEndUtc, op::ModifierOp, r#type::ModifierType},
        },
        needs_home::BaseValuePair,
        runtime::DataBoxSlotId,
        utils::boxes::data_box::DataBox,
    };

    fn make_test_component(slot_id: DataBoxSlotId) -> Component {
        Component::new(slot_id)
            .add_name("Test Component")
            .add_description("Test")
            .build()
            .expect("Test component should be valid")
    }

    #[test]
    fn basic_component_builder() {
        let mut map = SlotMap::new();
        let real_id = map.insert("test");
        let component = Component::new(real_id)
            .add_name("Test Component")
            .add_description("Test")
            .build();
        assert!(component.is_ok());
    }

    #[test]
    fn full_component() {
        let mut map = SlotMap::new();
        let real_id = map.insert(DataBox::new());
        let mut component = make_test_component(real_id);
        let modifier = Modifier {
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(10, 0))],
            kind: ModifierType::Percentage,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Add,
        };
        let modifier_utc = ModifierWithEndUtc::new_no_end(modifier);
        let flat_modifier = Modifier {
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(2, 0))],
            kind: ModifierType::Flat,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Add,
        };
        let flat_modifier = ModifierWithEndUtc::new_no_end(flat_modifier);
        let db = map
            .get_mut(*component.get_data_box())
            .expect("Id must exist");
        let stb = db.get_stat_box_mut();
        stb.add_modifier(modifier_utc.clone());
        stb.add_modifier(flat_modifier);
        let datas = stb.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(6, 0));
        component.add_component(make_test_component(real_id), db);
        let stb = db.get_stat_box_mut();
        let datas = stb.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(9, 0));
        let capability = Capability::new()
            .add_name("Test Capability")
            .add_description("Test")
            .add_flag(SPACE_FLIGHT.to_string())
            .add_inherent_modifier(modifier_utc)
            .build();
        assert!(capability.is_ok());
        let capability = capability.unwrap();
        component.add_capability(capability, db);
        let stb = db.get_stat_box_mut();
        let datas = stb.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(12, 0));
    }

    #[test]
    fn basic_component_datas() {
        let mut map = SlotMap::new();
        let real_id = map.insert(DataBox::new());
        let component = make_test_component(real_id);
        let datas = map
            .get_mut(*component.get_data_box())
            .unwrap()
            .get_stat_box_mut()
            .get_full_stats();
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(2, 0));

        let modifier = Modifier {
            impacted_base_value_pairs: vec![BaseValuePair::new("health", HpFloat::new(5, 1))],
            kind: ModifierType::Percentage,
            duration: None,
            adds_flags: None,
            required_flags: None,
            op: ModifierOp::Mul,
        };
        let modifier = ModifierWithEndUtc::new_no_end(modifier);
        let stb = map
            .get_mut(*component.get_data_box())
            .expect("Id must exist")
            .get_stat_box_mut();
        stb.add_modifier(modifier);
        let datas = stb.get_full_stats();
        assert!(datas.contains_key("health"));
        assert!(datas.get("health").unwrap().as_hp_float().unwrap() == &HpFloat::new(3, 0));
    }

    #[test]
    fn builder_validation() {
        let mut map = SlotMap::new();
        let real_id = map.insert(DataBox::new());
        let component = Component::new(real_id).add_description("Test").build();
        assert!(component.is_err());
        let component = Component::new(real_id).add_name("Test Component").build();
        assert!(component.is_err());
        let component = Component::new(SlotMapIndex::new_dummy())
            .add_name("Test Component")
            .add_description("Test")
            .build();
        assert!(component.is_err());
    }
}
