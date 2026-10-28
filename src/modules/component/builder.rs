use crate::{
    error::{SSCCResult, new_component_builder_error},
    modules::component::{Capability, Component},
    utils::boxes::data_box::DataBox,
};
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ComponentBuilder {
    inner: Component,
}

impl ComponentBuilder {
    pub fn new(component: Component) -> Self {
        Self { inner: component }
    }
    pub fn add_name<S: Into<String>>(mut self, name: S) -> Self {
        self.inner.name = name.into();
        self
    }
    pub fn add_description<S: Into<String>>(mut self, description: S) -> Self {
        self.inner.description = description.into();
        self
    }
    pub fn add_component(mut self, component: Component, data_box_ref: &mut DataBox) -> Self {
        self.inner.add_component(component, data_box_ref);
        self
    }
    pub fn add_capability(mut self, capability: Capability, data_box_ref: &mut DataBox) -> Self {
        self.inner.add_capability(capability, data_box_ref);
        self
    }
    pub fn build(self) -> SSCCResult<Component> {
        let mut errs_encountered = Vec::new();

        if self.inner.name.is_empty() {
            errs_encountered.push("Name cannot be empty");
        }
        if self.inner.description.is_empty() {
            errs_encountered.push("Description cannot be empty");
        }
        if self.inner.data_box_id.is_dummy() {
            errs_encountered.push("Data box slot id cannot be a dummy");
        }

        if errs_encountered.len() == 0 {
            Ok(self.inner)
        } else {
            let mut err = new_component_builder_error();
            for err_enc in errs_encountered {
                err.push_ctx(err_enc);
            }
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use athena::slot_map::{SlotMap, SlotMapIndex};

    use crate::modules::component::Component;
    #[test]
    fn basics_component_builder() {
        let mut slot = SlotMap::new();
        let real_id = slot.insert("test");
        let dummy_id = SlotMapIndex::new_dummy();
        let component = Component::new(real_id)
            .add_name("test")
            .add_description("test desc")
            .build();
        assert!(component.is_ok());
        let component = Component::new(dummy_id)
            .add_name("test")
            .add_description("test desc")
            .build();
        assert!(component.is_err());
        let component = Component::new(real_id).add_name("test").build();
        assert!(component.is_err());
        let component = Component::new(real_id).add_description("test desc").build();
        assert!(component.is_err());
    }
}
