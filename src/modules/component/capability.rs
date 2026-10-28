use horae::Utc;

use crate::{
    consts::flags::INTERNAL_NONE,
    error::{SSCCResult, new_capability_builder_error},
    modules::modifier::ModifierWithEndUtc,
    needs_home::BaseValuePair,
    utils::boxes::data_box::flag_box::Flag,
};

/// Capability of a component
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Capability {
    name: String,
    description: String,
    flag: Flag,
    impacted_base_value_pair: Option<BaseValuePair>,
    inherent_modifiers: Vec<ModifierWithEndUtc>,
}

impl Capability {
    /// Create a new capability
    ///
    /// Initializes as a builder, call `.build()` when finished to validate the capability.
    ///
    /// # Required Fields
    /// - `name` via [CapabilityBuilder::add_name]
    /// - `description` via [CapabilityBuilder::add_description]
    /// - `kind` via [CapabilityBuilder::add_kind] that is not [CapabilityKind::None]
    pub fn new() -> CapabilityBuilder {
        CapabilityBuilder::new()
    }
    fn add_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }
    fn add_description<S: Into<String>>(&mut self, description: S) {
        self.description = description.into();
    }
    fn add_flag(&mut self, flag: Flag) {
        self.flag = flag;
    }
    fn add_impacted_base_value_pair(&mut self, base_value_pair: BaseValuePair) {
        self.impacted_base_value_pair = Some(base_value_pair);
    }
    fn add_inherent_modifier(&mut self, modifier: ModifierWithEndUtc) {
        self.inherent_modifiers.push(modifier);
    }
    pub fn has_flag(&self, flag: &Flag) -> bool {
        &self.flag == flag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn flag(&self) -> &Flag {
        &self.flag
    }
    pub fn impacted_base_value_pair(&self) -> &Option<BaseValuePair> {
        &self.impacted_base_value_pair
    }
    pub fn inherent_modifiers(&self) -> &Vec<ModifierWithEndUtc> {
        &self.inherent_modifiers
    }
    pub fn update_inherent_modifiers(&mut self, now_utc: &Utc) {
        self.inherent_modifiers = drop_modifiers_past_date(&self.inherent_modifiers, now_utc);
    }
}

fn drop_modifiers_past_date(
    modifiers: &Vec<ModifierWithEndUtc>,
    now_utc: &Utc,
) -> Vec<ModifierWithEndUtc> {
    modifiers
        .iter()
        .filter(|modifier| modifier.end_utc > Some(*now_utc))
        .cloned()
        .collect()
}

pub struct CapabilityBuilder {
    inner: Capability,
}

impl CapabilityBuilder {
    pub fn new() -> CapabilityBuilder {
        CapabilityBuilder {
            inner: Capability {
                name: String::new(),
                description: String::new(),
                flag: INTERNAL_NONE.to_string(),
                impacted_base_value_pair: None,
                inherent_modifiers: Vec::new(),
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
    pub fn add_flag(mut self, flag: Flag) -> Self {
        self.inner.add_flag(flag);
        self
    }
    pub fn add_impacted_base_value_pair(mut self, base_value_pair: BaseValuePair) -> Self {
        self.inner.add_impacted_base_value_pair(base_value_pair);
        self
    }
    pub fn add_inherent_modifier(mut self, modifier: ModifierWithEndUtc) -> Self {
        self.inner.add_inherent_modifier(modifier);
        self
    }
    pub fn build(self) -> SSCCResult<Capability> {
        let mut errs_encountered = Vec::new();

        if self.inner.name.is_empty() {
            errs_encountered.push("Name cannot be empty");
        }
        if self.inner.description.is_empty() {
            errs_encountered.push("Description cannot be empty");
        }
        if self.inner.flag == INTERNAL_NONE {
            errs_encountered.push("Flag cannot be None");
        }
        if errs_encountered.len() == 0 {
            Ok(self.inner)
        } else {
            let mut err = new_capability_builder_error();
            for err_enc in errs_encountered {
                err.push_ctx(err_enc);
            }
            Err(err)
        }
    }
}
