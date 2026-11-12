use athena::float::HpFloat;

use crate::consts::{
    characters::{
        skills::{
            ConstSkill,
            levels::{Level, SKILL_LEVEL_TOTAL_AMOUNT, calc_final_point_cost_for_level},
        },
        traits::{AffinityTarget, CharacterTrait},
    },
    normalise_bvp_name,
};

pub type CharacterSkillRegistryId = usize;

pub struct CharacterSkill {
    pub skill: &'static ConstSkill,
    affinity: HpFloat,
    points: HpFloat,
    level: Level,
    /// Increments on each idle check (e.g. weekly/monthly); resets to 0 on use.
    unused_counter: u8,
    lost_rust_this_sim_tick: bool,
    /// Current rust stacks (0..=max_rust).
    rust: u8,
    /// Calculated from affinity only. Always updated when affinity changes
    max_rust: u8,
}

impl CharacterSkill {
    pub fn update_from_traits(&mut self, traits: &[&'static CharacterTrait]) {
        self.affinity = self.calc_affinity(traits);
        self.max_rust = calc_max_rust_for_affinity(self.affinity);
        if self.rust > self.max_rust {
            self.rust = self.max_rust
        }
    }
    pub fn simulation_tick(&mut self) {
        self.unused_counter += 1;
        if self.unused_counter >= 6 {
            self.unused_counter = 0;
            self.rust += 1;
            if self.rust > self.max_rust {
                self.rust = self.max_rust
            }
        }
        self.lost_rust_this_sim_tick = false;
    }
    pub fn add_points(&mut self, points: HpFloat) {
        let new_points = self.points + points;
        let needed_points = calc_final_point_cost_for_level(self.level);
        if needed_points > new_points {
            self.points = new_points;
        } else {
            self.points = new_points - needed_points;
            self.level = self.level.level_up();
        }
    }
    pub fn skill_used(&mut self) {
        self.unused_counter = 0;
        if !self.lost_rust_this_sim_tick {
            self.rust = self.rust.saturating_sub(1);
            self.lost_rust_this_sim_tick = true;
        }
    }
    pub fn level_up_immediately(&mut self) {
        self.reset_current_level();
        self.level = self.level.level_up();
    }
    /// Resets the progress of the current level to 0 across the board.
    pub fn reset_current_level(&mut self) {
        self.points = HpFloat::new_zero();
        self.lost_rust_this_sim_tick = true;
        self.rust = 0;
        self.unused_counter = 0;
    }
    pub fn set_level(&mut self, new_level: Level) {
        self.reset_current_level();
        self.level = new_level;
    }
    /// Hard reset the rust subsystem.
    ///
    /// Consider using [`CharacterSkill::skill_used`] if applicable.
    pub fn reset_rust(&mut self) {
        self.unused_counter = 0;
        self.rust = 0;
        self.lost_rust_this_sim_tick = false;
    }
    pub fn is_rusty(&self) -> bool {
        if self.rust == 0 { false } else { true }
    }
    pub fn level(&self) -> &Level {
        &self.level
    }
    pub fn affinity(&self) -> &HpFloat {
        &self.affinity
    }
    fn calc_affinity(&self, traits: &[&'static CharacterTrait]) -> HpFloat {
        let mut tmp = HpFloat::new_zero();
        for tr in traits.iter() {
            for affinity in tr.affinities() {
                match affinity.target {
                    AffinityTarget::None => {
                        debug_assert!(
                            false,
                            "Affinity with no defined affinity target encountered!\nTrait: {tr:?}; Affinity: {affinity:?}"
                        )
                    }
                    AffinityTarget::Skill(skill_name) => {
                        if skill_name == self.skill.name {
                            tmp = tmp + affinity.delta;
                        }
                    }
                    AffinityTarget::BaseValue(bv_name) => {
                        let normalised_bv_names = normalise_bvp_name(bv_name);
                        for name in normalised_bv_names {
                            if let Some(master) = self.skill.master {
                                if master.base_value_name == name {
                                    tmp = tmp + (affinity.delta * 1.0);
                                }
                            } else {
                                for secondary in self.skill.secondary_skills() {
                                    if secondary.base_value_name == name {
                                        tmp = tmp + (affinity.delta * 0.25);
                                    }
                                }
                            }
                        }
                    }
                    AffinityTarget::AllSkills => {
                        tmp = tmp + affinity.delta;
                    }
                }
            }
        }
        tmp
    }
}

fn scale_affinity_for_rust(affinity: HpFloat) -> HpFloat {
    let out;
    if affinity <= HpFloat::new_zero() {
        out = HpFloat::new_zero();
    } else if affinity >= HpFloat::new(1, 0) {
        out = HpFloat::new(SKILL_LEVEL_TOTAL_AMOUNT as i128, 0);
    } else {
        // In range aff: > 0.0 && < 1.0
        out = HpFloat::new(SKILL_LEVEL_TOTAL_AMOUNT as i128, 0) * affinity;
    }
    out
}

fn calc_max_rust_for_affinity(affinity: HpFloat) -> u8 {
    let tmp = HpFloat::new(SKILL_LEVEL_TOTAL_AMOUNT as i128, 0) - scale_affinity_for_rust(affinity);
    let tmp = tmp.get_integer_part();
    debug_assert!(tmp >= 0, "Negative rust! '{tmp}'");
    debug_assert!(tmp <= 255, "Over u8::MAX rust! '{tmp}'");
    if tmp < 0 || tmp > 256 { 0 } else { tmp as u8 }
}
