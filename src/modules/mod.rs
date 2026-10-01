pub mod component;
pub mod science;
pub mod ship_class;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Capability {
    SpaceDocking,
    SpaceFlight,
    AtmosphericFlight,
    InterstellarFlight,
}
