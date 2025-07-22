use std::collections::BTreeMap;


pub struct State {
    pub run_game: bool,
    pub startup: bool,
    pub new_game: bool,
    pub game_state: GameState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            startup: true,
            run_game: false,
            new_game: false,
            game_state: GameState::default(),
        }
    }
}

pub struct GameState {
    // I know, unix timestamp is way overkill - u16 is only 18h of gameplay though
    pub increment_timer: u32,
    pub company_name: String,
    pub money: u32,
    pub reputation: u32,
    pub research_state: ResearchState,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            company_name: String::from("Space Ship Construction Company"),
            increment_timer: 0,
            money: 0,
            reputation: 0,
            research_state: ResearchState::default(),
        }
    }
}

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
            available_technologies: generate_default_technologies(),
            locked_technologies: generate_default_locked_technologies(),
        }
    }
}

fn generate_default_technologies() -> BTreeMap<String, Technology> {
    let mut technologies = BTreeMap::new();
    technologies.insert(String::from("test"), Technology::default());
    technologies
}

fn generate_default_locked_technologies() -> BTreeMap<String, Technology> {
    let mut technologies = BTreeMap::new();
    technologies.insert(String::from("test"), Technology::default());
    technologies
}

pub struct Technology {
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub invested_research_points: u32,
    pub technology_type: TechnologyType,
    pub required_technologies: Vec<String>,
}

impl Default for Technology {
    fn default() -> Self {
        Self {
            name: String::from("test"),
            description: String::from("test"),
            cost: 0,
            invested_research_points: 0,
            technology_type: TechnologyType::HeavyEngineering,
            required_technologies: Vec::new(),
        }
    }
}

pub enum TechnologyType {
    HeavyEngineering,
    LightEngineering,
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

pub struct ResearchPoints {
    pub heavy_engineering_points: u32,
    pub light_engineering_points: u32,
    pub propulsion_points: u32,
    pub navigation_points: u32,
    pub communication_points: u32,
    pub weapons_points: u32,
    pub sensors_points: u32,
    pub shield_points: u32,
    pub armor_points: u32,
    pub hull_points: u32,
    pub cargo_points: u32,
    pub crew_points: u32,
    pub power_points: u32,
    pub life_support_points: u32,
    pub maintenance_points: u32,
}

impl Default for ResearchPoints {
    fn default() -> Self {
        Self {
            heavy_engineering_points: 0,
            light_engineering_points: 0,
            propulsion_points: 0,
            navigation_points: 0,
            communication_points: 0,
            weapons_points: 0,
            sensors_points: 0,
            shield_points: 0,
            armor_points: 0,
            hull_points: 0,
            cargo_points: 0,
            crew_points: 0,
            power_points: 0,
            life_support_points: 0,
            maintenance_points: 0,
        }
    }
}
