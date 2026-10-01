use aequa::Object;
use nemesis::NemesisError;

use crate::{
    error::{SSCCError, SSCCResult, new_component_builder_error},
    modules::{Capability, component::Component},
};
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ComponentBuilder {
    /// Name of the component
    name: String,
    /// Description
    description: String,
    /// Base values (no modifiers)
    base_values: Object,
    /// Any modifier that modifies the base values
    modifiers: Object,
    /// Any modifier that is added to the base values before applying modifiers
    flat_modifiers: Object,
    /// Any modifier that does not modify the base values
    unused_modifiers: Object,
    /// Any flat modifier that does not currently modify the base values
    unused_flat_modifiers: Object,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Child components
    components: Vec<Component>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        ComponentBuilder {
            name: String::new(),
            description: String::new(),
            base_values: Object::new(),
            modifiers: Object::new(),
            flat_modifiers: Object::new(),
            unused_modifiers: Object::new(),
            unused_flat_modifiers: Object::new(),
            capabilities: Vec::new(),
            components: Vec::new(),
        }
    }
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = description.into();
        self
    }
    pub fn add_modifier(mut self, modifier: Object) -> Self {
        let modifier_name = {
            if let Some(modifier_name) = modifier.get("name") {
                modifier_name
                    .into_string()
                    .expect("Modifier name is not a string")
            } else {
                todo!("No modifier name in modifier")
            }
        };
        if self.modifiers.contains_key(&modifier_name) {
            // Modifier already exists
            // Ignore for now
        } else {
            // Modifier does not exist
            let base_value_kind = {
                if let Some(base_value_kind) = modifier.get("base_value_kind") {
                    base_value_kind
                        .into_string()
                        .expect("Base value kind is not a string")
                } else {
                    todo!("No base value kind in modifier")
                }
            };
            let base_value_value = {
                if let Some(base_value_value) = modifier.get("base_value_value") {
                    base_value_value
                        .into_hp_float()
                        .expect("Base value value is not a number")
                } else {
                    // No base value found, push to unused modifiers
                    if !self.unused_modifiers.contains_key(&modifier_name) {
                        self.unused_modifiers
                            .insert(modifier_name.clone(), modifier);
                    }
                    return self;
                }
            };
            if let Some(base_value) = self.base_values.get_mut(&base_value_kind) {
                let base_value = base_value
                    .as_hp_float_mut()
                    .expect("Base value is not a number");
                *base_value = *base_value + base_value_value;
            } else {
                self.base_values
                    .insert(base_value_kind.clone(), base_value_value);
            }
            self.modifiers.insert(modifier_name, modifier);
        }
        self
    }
    pub fn add_flat_modifier(mut self, modifier: Object) -> Self {
        todo!("Really almost identical to add_modifier")
    }
    pub fn add_component(mut self, component: Component) -> Self {
        for (base_value_name, base_value_value) in component.base_values.iter() {
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
        for (_, modifier) in component.modifiers.iter() {
            if let Some(modifier) = modifier.into_object() {
                self = self.add_modifier(modifier);
            } else {
                todo!("Modifier is not an object")
            }
        }
        for (_, modifier) in component.flat_modifiers.iter() {
            if let Some(modifier) = modifier.into_object() {
                self = self.add_flat_modifier(modifier);
            } else {
                todo!("Modifier is not an object")
            }
        }
        for (unused_modifier_name, unused_modifier) in component.unused_modifiers.iter() {
            let unused_modifier = unused_modifier
                .into_object()
                .expect("Modifier is not an object");
            let base_value_kind = {
                if let Some(base_value_kind) = unused_modifier.get("base_value_kind") {
                    base_value_kind
                        .into_string()
                        .expect("Base value kind is not a string")
                } else {
                    todo!("No base value kind in modifier")
                }
            };
            if self.base_values.contains_key(&base_value_kind) {
                self = self.add_modifier(unused_modifier.clone());
            } else {
                // Modifier does not exist
                if !self.unused_modifiers.contains_key(unused_modifier_name) {
                    self.unused_modifiers
                        .insert(unused_modifier_name.clone(), unused_modifier);
                }
            }
        }
        for (unused_flat_modifier_name, unused_flat_modifier) in
            component.unused_flat_modifiers.iter()
        {
            let unused_flat_modifier = unused_flat_modifier
                .into_object()
                .expect("Modifier is not an object");
            let base_value_kind = {
                if let Some(base_value_kind) = unused_flat_modifier.get("base_value_kind") {
                    base_value_kind
                        .into_string()
                        .expect("Base value kind is not a string")
                } else {
                    todo!("No base value kind in modifier")
                }
            };
            if self.base_values.contains_key(&base_value_kind) {
                self = self.add_flat_modifier(unused_flat_modifier.clone());
            } else {
                // Modifier does not exist
                if !self
                    .unused_flat_modifiers
                    .contains_key(unused_flat_modifier_name)
                {
                    self.unused_flat_modifiers
                        .insert(unused_flat_modifier_name.clone(), unused_flat_modifier);
                }
            }
        }
        self.capabilities.extend(component.capabilities.clone());
        self.components.push(component);
        self
    }
    pub fn build(self) -> SSCCResult<Component> {
        let mut errs_encountered = Vec::new();
        if self.name.is_empty() {
            errs_encountered.push("Name cannot be empty");
        }
        if self.description.is_empty() {
            errs_encountered.push("Description cannot be empty");
        }
        if self.base_values.len() == 0 {
            errs_encountered.push("No base values found. At least one is required");
        }

        if errs_encountered.len() == 0 {
            Ok(Component {
                name: self.name,
                description: self.description,
                base_values: self.base_values,
                modifiers: self.modifiers,
                flat_modifiers: self.flat_modifiers,
                unused_modifiers: self.unused_modifiers,
                unused_flat_modifiers: self.unused_flat_modifiers,
                capabilities: self.capabilities,
                components: self.components,
            })
        } else {
            let mut err = new_component_builder_error();
            for err_enc in errs_encountered {
                err.push_ctx(err_enc);
            }
            Err(err)
        }
    }
}
