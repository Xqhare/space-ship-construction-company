use research_state::ResearchState;


pub mod research_state;

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
    // This supports 584 billion years, way to large, but better than u32 (136 years)  (assuming only 365 day years with perfect 24h per day)
    pub increment_timer: u64,
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
