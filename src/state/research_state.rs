use std::collections::BTreeMap;

use crate::game::tech::{default::{generate_locked_technologies, generate_available_technologies}, Technology};

use super::ResearchPoints;

pub struct ResearchState {
    pub research_points: ResearchPoints,
    pub researched_technologies: BTreeMap<String, Technology>,
    pub available_technologies: BTreeMap<String, Technology>,
    pub locked_technologies: BTreeMap<String, Technology>,
}

impl Default for ResearchState {
    fn default() -> Self {
        Self {
            research_points: ResearchPoints::default(),
            researched_technologies: BTreeMap::new(),
            available_technologies: generate_available_technologies(),
            locked_technologies: generate_locked_technologies(),
        }
    }
}
