pub mod component;
pub mod modifiers;
pub mod science;
pub mod ship_class;
pub mod stat_box;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Capability {
    SpaceDocking,
    SpaceFlight,
    AtmosphericFlight,
    InterstellarFlight,
}
