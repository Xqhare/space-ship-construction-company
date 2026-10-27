use athena::slot_map::SlotMapIndex;

use crate::{
    error::{SSCCResult, new_ship_class_builder_error},
    modules::component::{Component, capability::Capability},
    runtime::DataBoxSlotId,
};

#[non_exhaustive]
pub struct ShipClass {
    /// Name of the class
    name: String,
    /// Description
    description: String,
    /// Statistic properties of the class
    data_box_slot_id: DataBoxSlotId,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Components that make up the class
    components: Vec<Component>,
}

impl ShipClass {
    pub fn new() -> ShipClassBuilder {
        ShipClassBuilder::new()
    }
    pub fn add_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }
    pub fn add_description<S: Into<String>>(&mut self, description: S) {
        self.description = description.into();
    }
    pub fn add_data_box(&mut self, stat_box_slot_id: DataBoxSlotId) {
        self.data_box_slot_id = stat_box_slot_id;
    }
    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.push(capability);
    }
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}

pub struct ShipClassBuilder {
    inner: ShipClass,
}

impl ShipClassBuilder {
    pub fn new() -> ShipClassBuilder {
        ShipClassBuilder {
            inner: ShipClass {
                name: String::new(),
                description: String::new(),
                data_box_slot_id: SlotMapIndex::new_dummy(),
                capabilities: Vec::new(),
                components: Vec::new(),
            },
        }
    }
    pub fn add_name<S: Into<String>>(mut self, name: S) -> Self {
        self.inner.add_name(name);
        self
    }
    pub fn add_description<S: Into<String>>(mut self, description: S) -> Self {
        self.inner.add_description(description);
        self
    }
    pub fn add_data_box(mut self, data_box_id: DataBoxSlotId) -> Self {
        self.inner.add_data_box(data_box_id);
        self
    }
    pub fn add_capability(mut self, capability: Capability) -> Self {
        self.inner.add_capability(capability);
        self
    }
    pub fn add_component(mut self, component: Component) -> Self {
        self.inner.add_component(component);
        self
    }
    pub fn build(self) -> SSCCResult<ShipClass> {
        let mut errs_encountered = Vec::new();

        if self.inner.name.is_empty() {
            errs_encountered.push("Name cannot be empty");
        }
        if self.inner.description.is_empty() {
            errs_encountered.push("Description cannot be empty");
        }
        if self.inner.data_box_slot_id.is_dummy() {
            errs_encountered.push("Data box slot id cannot be dummy");
        }
        if errs_encountered.len() == 0 {
            Ok(self.inner)
        } else {
            let mut err = new_ship_class_builder_error();
            for err_enc in errs_encountered {
                err.push_ctx(err_enc);
            }
            Err(err)
        }
    }
}
