pub struct Technology {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub invested_research_points: u32,
    pub technology_type: TechnologyType,
    pub required_technologies: Vec<u32>,
}

impl Default for Technology {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("test"),
            description: String::from("test"),
            cost: 0,
            invested_research_points: 0,
            technology_type: TechnologyType::Engineering,
            required_technologies: Vec::new(),
        }
    }
}

pub enum TechnologyType {
    Engineering,
    AtmosphericFlight,
    ShipDesign,
    Propulsion,
    Navigation,
    Communication,
    Weapons,
    Sensors,
    Shield,
    Armor,
    Hull,
    Cargo,
    Crew,
    Power,
    LifeSupport,
    Maintenance,
}

