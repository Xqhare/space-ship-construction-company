use std::collections::BTreeMap;

use super::Technology;

pub fn generate_available_technologies() -> BTreeMap<u32, Technology> {
    let mut technologies = BTreeMap::new();
    technologies.insert(0, Technology::default());
    technologies
}

pub fn generate_locked_technologies() -> BTreeMap<u32, Technology> {
    let mut technologies = BTreeMap::new();
    technologies.insert(0, Technology::default());
    technologies
}

fn available_tech() -> [(u32, Technology); 1] {
    let available_tech: [(u32, Technology); 1] = [
        (0, Technology {
            id: 0,
            name: String::from("test"),
            description: String::from("test"),
            cost: 0,
            invested_research_points: 0,
            technology_type: super::TechnologyType::Engineering,
            required_technologies: Vec::new(),
        }),
    ];
    available_tech
}
