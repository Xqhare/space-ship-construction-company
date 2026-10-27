use crate::modules::event::event_queue::tick::{simulation::SimulationTick, special::SpecialTick};

pub mod simulation;
pub mod special;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tick {
    Simulation(SimulationTick),
    Special(SpecialTick),
}
