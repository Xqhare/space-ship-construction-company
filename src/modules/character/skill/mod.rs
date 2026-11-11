use athena::float::HpFloat;

use crate::{
    consts::characters::{
        skills::{
            ConstSkill,
            levels::{
                Level, SKILL_LEVEL_TOTAL_AMOUNT, SKILL_LEVEL_ULTRA_AMOUNT,
                calc_final_point_cost_for_level,
            },
        },
        traits::CharacterTrait,
    },
    modules::modifier::r#type::ModifierType,
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
    pub fn level_up_immediately(&mut self) {}
    pub fn set_level(&mut self, new_level: Level) {}
    /// Hard reset the rust subsystem.
    ///
    /// Consider using [`CharacterSkill::skill_used`] if applicable.
    pub fn reset_rust(&mut self) {
        self.unused_counter = 0;
        self.rust = 0;
        self.lost_rust_this_sim_tick = false;
    }
    pub fn is_rusty(&self) -> bool {
        if self.rust == 0 { true } else { false }
    }
    pub fn level(&self) -> &Level {
        &self.level
    }
    pub fn affinity(&self) -> &HpFloat {
        &self.affinity
    }
    // TODO: See comment
    fn calc_affinity(&self, traits: &[&'static CharacterTrait]) -> HpFloat {
        let mut tmp = HpFloat::new_zero();
        for tr in traits.iter() {
            for impact in tr.affected_base_value_pairs() {
                // Well this is some kind of bs.
                // Either I add an affinity field to impact OR I come up with an actual
                // system of normalising master & secondary skills into affinity.
                //
                // Issue: Take 'Genius' with no master skill; This would add very little
                // affinity to each skill affected by the trait. Not desirable
                //
                // # Option 1:
                // Add a affinity field to CharacterTrait.
                // Probably as a `Vec<(base_value_name, affinity)>` (`Vec<&'static str,
                // HpFloat`)
                //
                // Then just normalise the base_name and add affinity to the total.
                //
                // Pro: Simple
                // Con: Little flexibility
                //
                // # Option 2:
                // Add a new field of type `[TraitAffinityImpact; MAX_AFF_IMPACTS]`.
                //
                // ```
                //#[derive(Debug, Clone, Copy)]
                //   pub struct TraitAffinityImpact {
                //       pub base_value_name: &'static str,
                //       pub affinity: f32,
                //       pub kind: ModifierType,
                //       pub op: Op,
                //   }
                // ```
                //
                // The `kind` field (copied from TraitSkillImpact) could just be lost; Or
                // replaced with some other kind for more flexibility in defining
                // affinites on traits
                //
                // # Option 3:
                // Stick largely with the solution in code (differentiating between master
                // and secondary skills); Feels very band-aid-y to me though.
                if let Some(master) = self.skill.master
                    && master.base_value_name == impact.base_value_name
                {
                    match impact.kind {
                        ModifierType::Flat => tmp = tmp + impact.base_value_value,
                        ModifierType::Percentage => tmp = tmp + (impact.base_value_value * 100.0),
                    }
                }
                for secondaty in self.skill.secondary_skills() {
                    if secondaty.base_value_name == impact.base_value_name {
                        match impact.kind {
                            ModifierType::Flat => tmp = tmp + (impact.base_value_value / 4.0),
                            ModifierType::Percentage => {
                                tmp = tmp + ((impact.base_value_value * 100.0) / 4.0)
                            }
                        }
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
    debug_assert!(tmp > 0, "Negative rust! '{tmp}'");
    debug_assert!(tmp <= 256, "Over u8::MAX rust! '{tmp}'");
    if tmp < 0 || tmp > 256 { 0 } else { tmp as u8 }
}
