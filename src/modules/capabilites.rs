use aequa::hp_float::HpFloat;

use crate::{
    error::{SSCCResult, new_capability_builder_error},
    modules::modifiers::Modifier,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum CapabilityKind {
    None,
    Docking,
    SpaceFlight,
    AtmosphericFlight,
    InterstellarFlight,
    CivilianDataUplink,
    CivilianDataDownlink,
    MilitaryDataUplink,
    MilitaryDataDownlink,
    PositionHolding,
    ElectronicDefensiveSystems,
    ElectronicOffensiveSystems,
    ElectronicCountermeasures,
    /// Can be landed on something else
    Landing,
    /// Can be landed on
    Landable,
    CargoHold,
    Research,
    Scout,
    Mine,
    Harvest,
    Grow,
    /// Can give out supplies
    Supply,
    /// Can repair others
    Repair,
    /// Can repair itself
    RepairOwn,
    /// Can receive supplies
    Replenishable,
    Salvage,
    Passanger,
}

/// Capability of a component
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Capability {
    name: String,
    description: String,
    kind: CapabilityKind,
    base_value_name: Option<String>,
    base_value_value: Option<HpFloat>,
    inherent_modifiers: Vec<Modifier>,
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
    pub fn add_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }
    pub fn add_description<S: Into<String>>(&mut self, description: S) {
        self.description = description.into();
    }
    pub fn add_kind(&mut self, kind: CapabilityKind) {
        self.kind = kind;
    }
    pub fn add_base_value_pair<S: Into<String>>(&mut self, name: S, value: HpFloat) {
        self.base_value_name = Some(name.into());
        self.base_value_value = Some(value);
    }
    pub fn add_inherent_modifier(&mut self, modifier: Modifier) {
        self.inherent_modifiers.push(modifier);
    }
    pub fn is_kind(&self, kind: &CapabilityKind) -> bool {
        &self.kind == kind
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn kind(&self) -> &CapabilityKind {
        &self.kind
    }
    pub fn base_value_name(&self) -> &Option<String> {
        &self.base_value_name
    }
    pub fn base_value_value(&self) -> &Option<HpFloat> {
        &self.base_value_value
    }
    pub fn inherent_modifiers(&self) -> &Vec<Modifier> {
        &self.inherent_modifiers
    }
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
                kind: CapabilityKind::None,
                base_value_name: None,
                base_value_value: None,
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
    pub fn add_kind(mut self, kind: CapabilityKind) -> Self {
        self.inner.add_kind(kind);
        self
    }
    pub fn add_base_value_pair<S: Into<String>>(mut self, name: S, value: HpFloat) -> Self {
        self.inner.add_base_value_pair(name, value);
        self
    }
    pub fn add_inherent_modifier(mut self, modifier: Modifier) -> Self {
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
        if self.inner.kind == CapabilityKind::None {
            errs_encountered.push("Kind cannot be None");
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
