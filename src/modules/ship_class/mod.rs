use crate::modules::{capabilites::Capability, component::Component, stat_box::StatBox};

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
