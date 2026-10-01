use aequa::{Object, XffValue};

use crate::{
    error::{SSCCResult, new_component_builder_error},
    modules::{component::Component, modifiers::Modifier},
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
    pub fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.inner.add_modifier(modifier);
        self
    }
    pub fn add_component(mut self, component: Component) -> Self {
        self.inner.add_component(component);
        self
    }
    pub fn add_base_value<S: Into<String>>(mut self, key: S, value: XffValue) -> Self {
        self.inner.add_base_value(key, value);
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
        if !self.inner.stat_box.has_base_values() {
            errs_encountered.push("No base values found. At least one is required");
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

#[test]
fn basics_component_builder() {
    use aequa::xff;
    let component = Component::new()
        .add_name("test")
        .add_description("test")
        .add_base_value("weight", xff!(1.0))
        .build();
    assert!(component.is_ok());
    let component = Component::new()
        .add_name("test")
        .add_description("test")
        .build();
    assert!(component.is_err());
    let component = Component::new()
        .add_name("test")
        .add_base_value("weight", xff!(1.0))
        .build();
    assert!(component.is_err());
    let component = Component::new()
        .add_description("test")
        .add_base_value("weight", xff!(1.0))
        .build();
    assert!(component.is_err());
}
