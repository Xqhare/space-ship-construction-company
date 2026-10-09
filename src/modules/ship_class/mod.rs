use crate::{
    error::{SSCCResult, new_ship_class_builder_error},
    modules::{capabilites::Capability, component::Component, stat_box::StatBox},
};

#[non_exhaustive]
pub struct ShipClass {
    /// Name of the class
    name: String,
    /// Description
    description: String,
    /// Statistic properties of the class
    stat_box: StatBox,
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
    pub fn add_stat_box(&mut self, stat_box: StatBox) {
        self.stat_box = stat_box;
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
                stat_box: StatBox::new(),
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
    pub fn add_stat_box(mut self, stat_box: StatBox) -> Self {
        self.inner.add_stat_box(stat_box);
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
