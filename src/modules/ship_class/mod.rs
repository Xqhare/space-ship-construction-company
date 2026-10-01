use aequa::Object;

use crate::modules::{Capability, component::Component};

#[non_exhaustive]
pub struct ShipClass {
    /// Name of the class
    name: String,
    /// Description
    description: String,
    /// Base values (no modifiers)
    base_values: Object,
    /// Any modifier that modifies the base values
    modifiers: Object,
    /// Any modifier that does not modify the base values
    unused_modifiers: Object,
    /// Any capabilities
    capabilities: Vec<Capability>,
    /// Components that make up the class
    components: Vec<Component>,
}
