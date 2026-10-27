use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;

use crate::{
    error::new_xff_serde_error,
    modules::event::event_queue::tick::{
        simulation::{SimulationTick, SimulationTickKind},
        special::{SpecialTick, SpecialTickKind},
    },
};

pub mod simulation;
pub mod special;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tick {
    Simulation(SimulationTick),
    Special(SpecialTick),
}

impl Tick {
    pub fn get_time(&self) -> Utc {
        match self {
            Tick::Simulation(sim) => sim.time,
            Tick::Special(spec) => spec.time,
        }
    }
}

impl XffSerde for Tick {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Tick::Simulation(sim) => obj.insert("simulation", sim.to_xff()),
            Tick::Special(spec) => obj.insert("special", spec.to_xff()),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(sim) = obj.get("simulation") {
                Ok(Tick::Simulation(SimulationTick::from_xff(sim)?))
            } else if let Some(spec) = obj.get("special") {
                Ok(Tick::Special(SpecialTick::from_xff(spec)?))
            } else {
                Err(new_xff_serde_error("Unable to load Tick from XFF")
                    .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(
                new_xff_serde_error("Unable to load Tick from XFF, not an object")
                    .add_ctx(format!("Value: {xff:?}")),
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TickKind {
    Simulation(SimulationTickKind),
    Special(SpecialTickKind),
}
