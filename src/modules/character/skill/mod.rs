use athena::float::HpFloat;

use crate::consts::{
    characters::{
        skills::{
            ConstSkill,
            levels::{
                Level, SKILL_LEVEL_HIGH_AMOUNT, SKILL_LEVEL_LOW_AMOUNT, SKILL_LEVEL_MEDIUM_AMOUNT,
                SKILL_LEVEL_TOTAL_AMOUNT, calc_final_point_cost_for_level,
            },
            registries::get_skill_from_registry_by_name,
        },
        traits::{AffinityTarget, CharacterTrait},
    },
    normalise_bvp_name,
};

#[cfg(test)]
pub mod tests;

pub mod xff;

/// Dynamic per-character state for a [`ConstSkill`], tracking affinity, level progression,
/// and skill atrophy ("rust").
///
/// # Expected Usage & Unenforced Invariants
///
/// ## 1. Affinity (`affinity: HpFloat`)
/// - **Expected Range (`0.0..=1.0`, unenforced / soft-capped)**:
///   Calculated by [`CharacterSkill::update_from_traits`] starting from a base of `0.0125` (`1.25%`)
///   and summing matching [`TraitAffinityImpact`](crate::consts::characters::traits::TraitAffinityImpact)
///   deltas from the character's traits:
///   - [`AffinityTarget::Skill`] and [`AffinityTarget::AllSkills`]: `+1.0 * delta`
///   - [`AffinityTarget::BaseValue`] (expanded via [`normalise_bvp_name`]): `+1.0 * delta` when
///     matching `skill.master`, and `+0.25 * delta` for each matching entry in
///     [`ConstSkill::secondary_skills`].
/// - **Boundary & Out-of-Range Behavior**:
///   - `affinity <= 0.0`: Clamped to `0.0` when computing `max_rust` (resulting in
///     `max_rust = SKILL_LEVEL_TOTAL_AMOUNT = 33`), but **not** clamped in
///     [`CharacterSkill::add_points`] or
///     [`SkillStatImpact::calc_final_stat_impact`](crate::consts::characters::skills::SkillStatImpact::calc_final_stat_impact).
///     A skill with `<= 0.0` affinity cannot gain points (or loses points if negative) **and** produces
///     zero or negative stat output.
///   - `affinity >= 1.0`: Clamped to `1.0` when computing `max_rust` (resulting in `max_rust = 0`,
///     making the skill completely immune to rust), but **not** clamped in
///     [`CharacterSkill::add_points`] or `calc_final_stat_impact` (learning rate and stat output
///     continue scaling linearly above `1.0`).
/// - **Caching**: `affinity` and `max_rust` are cached on this struct for zero per-tick overhead.
///   Callers must invoke [`CharacterSkill::update_from_traits`] whenever the owning character's
///   traits change.
///         - Note: Cache may be a misnomer as this is the only place `affinity` or the `rust_max` of
///         a characters skill is stored. They could both be calculated from the other held data
///         though on demand, so in a way they are just cached values? Naming remains hard.
///
/// ## 2. Rust Subsystem (`rust`, `max_rust`, `unused_counter`, `lost_rust_this_sim_tick`)
/// - Models skill atrophy from non-use without deleting earned [`Level`] milestones:
///   - [`CharacterSkill::simulation_tick`] increments `unused_counter`; every `6` consecutive
///     simulation ticks without use resets `unused_counter` to `0` and adds `1` stack of `rust`
///     (capped at `max_rust`), and resets `lost_rust_this_sim_tick` to `false`.
///   - [`CharacterSkill::skill_used`] resets `unused_counter` to `0` and removes at most `1` stack
///     of `rust` per simulation tick (guarded by `lost_rust_this_sim_tick`).
///   - [`CharacterSkill::level`] returns the permanently earned [`Level`], whereas
///     [`CharacterSkill::effective_level`] returns `level.level_down_by(rust)` (clamped at
///     `Level::Low(0)`).
///
/// ## 3. Point Progression (`points`, `add_points`)
/// - [`CharacterSkill::add_points`] scales incoming raw points by `*self.affinity()` and compares
///   against [`calc_final_point_cost_for_level(self.level)`](calc_final_point_cost_for_level).
/// - Assumes a single `add_points` call does not exceed multiple levels' worth of points at once:
///   when `new_points >= needed_points`, it subtracts `needed_points` once and advances `self.level`
///   by a single step via [`Level::level_up`].
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CharacterSkill {
    pub skill: &'static ConstSkill,
    affinity: HpFloat,
    points: HpFloat,
    level: Level,
    /// Increments on each idle simulation tick; resets to `0` on use or every `6` ticks when adding rust.
    unused_counter: u8,
    lost_rust_this_sim_tick: bool,
    /// Current rust stacks (`0..=max_rust`).
    rust: u8,
    /// Maximum rust stacks (`0..=SKILL_LEVEL_TOTAL_AMOUNT`), derived from `affinity` whenever traits update.
    max_rust: u8,
}

// I may be taking the advice to 'minimise magic numbers as much as possible' a little too literally
const UNUSED_COUNTER_FLOOR: u8 = 0;
const UNUSED_COUNTER_INCREMENT: u8 = 1;
const UNUSED_COUNTER_ADD_RUST_TRIGGER: u8 = 6;
const RUST_FLOOR: u8 = 0;
const RUST_INCREMENT: u8 = 1;

impl CharacterSkill {
    /// Creates a new `CharacterSkill` at `Level::Low(0)` with `0` points and `0` rust,
    /// initializing `affinity` and `max_rust` from `traits`.
    pub fn new(skill: &'static ConstSkill, traits: &[&'static CharacterTrait]) -> CharacterSkill {
        let mut tmp = CharacterSkill {
            skill,
            affinity: HpFloat::new_zero(),
            points: HpFloat::new_zero(),
            level: Level::Low(0),
            unused_counter: UNUSED_COUNTER_FLOOR,
            lost_rust_this_sim_tick: false,
            rust: RUST_FLOOR,
            max_rust: RUST_FLOOR,
        };
        tmp.update_from_traits(traits);
        tmp
    }
    /// Normalizes the skill's starting [`Level`] based on the character's `iq`, `age_in_years`,
    /// and current `affinity` (resets current level progress and rust).
    ///
    /// # Formula & Expected Ranges
    /// - Expects `iq` in `[50.0, 149.0]` (`normalised_iq = (iq - 50.0) / 100.0`) and `age_in_years > 0.0`.
    /// - Computes `base_level = trunc(20 * ((iq - 50) / 100))` and
    ///   `final_level = trunc(base_level * (1 + (age_in_years / 100) + affinity))`.
    /// - Most characters land between `Level::Low(0)` and `Level::Medium(9)`.
    /// - Hard-capped at ordinal `29` (`Level::High(9)`); [`Level::Ultra`] is never produced by
    ///   this method.
    pub fn normalise_char_skill_from_char(&mut self, iq: HpFloat, age_in_years: HpFloat) {
        let normalised_iq = iq - 50.0; // => 0..99
        let niq_perc = normalised_iq / 100.0;
        let capped: HpFloat =
            HpFloat::cast((SKILL_LEVEL_LOW_AMOUNT + SKILL_LEVEL_MEDIUM_AMOUNT) as u64) * niq_perc;
        let base_level = HpFloat::cast(capped.get_integer_part());

        let affinity = self.affinity();

        let final_affinity = (age_in_years / 100.0) + *affinity;
        let final_level: HpFloat = base_level + (base_level * final_affinity);
        let final_level = final_level.get_integer_part().max(0);
        debug_assert!(
            final_level <= u8::MAX as i128,
            "Final calculated level is larger than u8::MAX!"
        );
        let final_level = if final_level
            >= (SKILL_LEVEL_LOW_AMOUNT + SKILL_LEVEL_MEDIUM_AMOUNT + SKILL_LEVEL_HIGH_AMOUNT)
                as i128
        {
            (SKILL_LEVEL_LOW_AMOUNT + SKILL_LEVEL_MEDIUM_AMOUNT + SKILL_LEVEL_HIGH_AMOUNT - 1) as u8
        } else {
            final_level as u8
        };
        self.set_level(Level::from_ordinal(final_level as u8));
    }
    /// Construct a skill using a skill_name.
    ///
    /// Returns `None` if the skill_name is not found in any skill registry
    pub fn from_skill(
        skill_name: &'static str,
        traits: &[&'static CharacterTrait],
    ) -> Option<CharacterSkill> {
        Some(CharacterSkill::new(
            get_skill_from_registry_by_name(skill_name)?,
            traits,
        ))
    }
    /// Updates the affinity and rust subsystem of the skill using the traits of the character
    /// possessing that skill.
    pub fn update_from_traits(&mut self, traits: &[&'static CharacterTrait]) {
        self.affinity = self.calc_affinity(traits);
        self.max_rust = calc_max_rust_for_affinity(self.affinity);
        if self.rust > self.max_rust {
            self.rust = self.max_rust
        }
    }
    pub fn simulation_tick(&mut self) {
        self.unused_counter += UNUSED_COUNTER_INCREMENT;
        if self.unused_counter >= UNUSED_COUNTER_ADD_RUST_TRIGGER {
            self.unused_counter = UNUSED_COUNTER_FLOOR;
            self.rust += RUST_INCREMENT;
            if self.rust > self.max_rust {
                self.rust = self.max_rust
            }
        }
        self.lost_rust_this_sim_tick = false;
    }
    pub fn add_points(&mut self, points: HpFloat) {
        let points = points * *self.affinity();
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
        self.unused_counter = UNUSED_COUNTER_FLOOR;
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
        self.reset_rust();
        self.lost_rust_this_sim_tick = true;
    }
    pub fn set_level(&mut self, new_level: Level) {
        self.reset_current_level();
        self.level = new_level;
    }
    /// Hard reset the rust subsystem.
    ///
    /// Consider using [`CharacterSkill::skill_used`] if applicable.
    #[inline]
    pub fn reset_rust(&mut self) {
        self.unused_counter = UNUSED_COUNTER_FLOOR;
        self.rust = RUST_FLOOR;
        self.lost_rust_this_sim_tick = false;
    }
    pub fn is_rusty(&self) -> bool {
        if self.rust == RUST_FLOOR { false } else { true }
    }
    /// Returns the current level *without* rust applied
    pub fn level(&self) -> &Level {
        &self.level
    }
    /// Returns the current level *with* rust applied
    pub fn effective_level(&self) -> Level {
        self.level.level_down_by(self.rust)
    }
    pub fn affinity(&self) -> &HpFloat {
        &self.affinity
    }
    fn calc_affinity(&self, traits: &[&'static CharacterTrait]) -> HpFloat {
        let mut tmp: HpFloat = 0.0125.into(); // 1.25% base affinity
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
                            if let Some(master) = self.skill.master
                                && master.base_value_name == name
                            {
                                tmp = tmp + (affinity.delta * 1.0);
                            }
                            for secondary in self.skill.secondary_skills() {
                                if secondary.base_value_name == name {
                                    tmp = tmp + (affinity.delta * 0.25);
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
