use std::cmp::Ordering;

use athena::float::HpFloat;
use fortuna::Fortuna;
use nemesis::NemesisResultExt;
use tyche::RngApi;

use crate::error::{SSCCResult, new_internal_error};

// NOTE: Rust derives PartialOrd/Ord on enums in variant declaration order (Low < Medium < High < Ultra) and then compares the inner u8
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// All inner `u8` values must be `< LEVEL_AMOUNT_*` for their tier.
///
/// Implements `Eq` and `Ord`
pub enum Level {
    Low(u8),
    Medium(u8),
    High(u8),
    Ultra(u8),
}

impl Level {
    /// Cast the level into the range 0..TOTAL_SKILL_LEVEL_AMOUNT
    pub fn to_ordinal(&self) -> u8 {
        match self {
            Level::Low(level) => *level,
            Level::Medium(level) => level.saturating_add(SKILL_LEVEL_LOW_AMOUNT as u8),
            Level::High(level) => level
                .saturating_add(SKILL_LEVEL_LOW_AMOUNT as u8)
                .saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT as u8),
            Level::Ultra(level) => level
                .saturating_add(SKILL_LEVEL_LOW_AMOUNT as u8)
                .saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT as u8)
                .saturating_add(SKILL_LEVEL_HIGH_AMOUNT as u8),
        }
    }
    // TODO: As nice as range.contains is; there is probably a nicer & better way
    /// Normalises any passed in `ordinal` that is larger than the expected maximum value to
    /// the maximum Skill level.
    ///
    /// For intended behaviour, always pair with [`Level::to_ordinal`]
    pub fn from_ordinal(ordinal: u8) -> Level {
        if (0..SKILL_LEVEL_LOW_AMOUNT).contains(&(ordinal as usize)) {
            Level::Low(ordinal)
        } else if (SKILL_LEVEL_LOW_AMOUNT
            ..SKILL_LEVEL_LOW_AMOUNT.saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT))
            .contains(&(ordinal as usize))
        {
            Level::Medium(ordinal.saturating_sub(SKILL_LEVEL_LOW_AMOUNT as u8))
        } else if (SKILL_LEVEL_LOW_AMOUNT.saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT)
            ..SKILL_LEVEL_LOW_AMOUNT
                .saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT)
                .saturating_add(SKILL_LEVEL_HIGH_AMOUNT))
            .contains(&(ordinal as usize))
        {
            Level::High(
                ordinal
                    .saturating_sub(SKILL_LEVEL_LOW_AMOUNT as u8)
                    .saturating_sub(SKILL_LEVEL_MEDIUM_AMOUNT as u8),
            )
        } else if (SKILL_LEVEL_LOW_AMOUNT
            .saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT)
            .saturating_add(SKILL_LEVEL_HIGH_AMOUNT)
            ..SKILL_LEVEL_LOW_AMOUNT
                .saturating_add(SKILL_LEVEL_MEDIUM_AMOUNT)
                .saturating_add(SKILL_LEVEL_HIGH_AMOUNT)
                .saturating_add(SKILL_LEVEL_ULTRA_AMOUNT))
            .contains(&(ordinal as usize))
        {
            Level::Ultra(
                ordinal
                    .saturating_sub(SKILL_LEVEL_LOW_AMOUNT as u8)
                    .saturating_sub(SKILL_LEVEL_MEDIUM_AMOUNT as u8)
                    .saturating_sub(SKILL_LEVEL_HIGH_AMOUNT as u8),
            )
        } else {
            Level::Ultra(SKILL_LEVEL_ULTRA_AMOUNT.saturating_sub(1) as u8)
        }
    }
    pub fn level_up_by(&self, amount: u8) -> Level {
        let ordinal = self.to_ordinal();
        Level::from_ordinal(ordinal.saturating_add(amount))
    }
    pub fn level_down_by(&self, amount: u8) -> Level {
        let ordinal = self.to_ordinal();
        Level::from_ordinal(ordinal.saturating_sub(amount))
    }
    pub fn level_up(&self) -> Level {
        self.level_up_by(1)
    }
    pub fn level_down(&self) -> Level {
        self.level_down_by(1)
    }
}

// NOTE: Because of the per-level cost formula, the distance between tiers should always be COST * 2 at a minimum.
const POINT_COST_LOW_LEVEL: u16 = 128; // Lets see if this needs to change
const POINT_COST_MEDIUM_LEVEL: u16 = POINT_COST_LOW_LEVEL * 2;
const POINT_COST_HIGH_LEVEL: u16 = POINT_COST_MEDIUM_LEVEL * 4;
const POINT_COST_ULTRA_LEVEL: u16 = POINT_COST_HIGH_LEVEL * 6;

/// Formula:
/// final_points_cost_of_level = round_down((1 + (level / LEVEL_AMOUNT)) * POINT_COST)
pub fn calc_final_point_cost_for_level(level: Level) -> HpFloat {
    let (base_cost, level) = {
        match level {
            Level::Low(cur) => (
                HpFloat::new(POINT_COST_LOW_LEVEL as i128, 0),
                HpFloat::new(cur as i128, 0) / HpFloat::new(SKILL_LEVEL_LOW_AMOUNT as i128, 0),
            ),
            Level::Medium(cur) => (
                HpFloat::new(POINT_COST_MEDIUM_LEVEL as i128, 0),
                HpFloat::new(cur as i128, 0) / HpFloat::new(SKILL_LEVEL_MEDIUM_AMOUNT as i128, 0),
            ),
            Level::High(cur) => (
                HpFloat::new(POINT_COST_HIGH_LEVEL as i128, 0),
                HpFloat::new(cur as i128, 0) / HpFloat::new(SKILL_LEVEL_HIGH_AMOUNT as i128, 0),
            ),
            Level::Ultra(cur) => (
                HpFloat::new(POINT_COST_ULTRA_LEVEL as i128, 0),
                HpFloat::new(cur as i128, 0) / HpFloat::new(SKILL_LEVEL_ULTRA_AMOUNT as i128, 0),
            ),
        }
    };
    let tmp: HpFloat = (level + 1.0) * base_cost;
    debug_assert!(
        tmp >= HpFloat::new(POINT_COST_LOW_LEVEL as i128, 0),
        "Something went terribly wrong; Final level points cost for level '{level:?}' is lower than the lowest cost defined! Cost: '{tmp}' < '{POINT_COST_LOW_LEVEL}'"
    );
    if tmp <= HpFloat::new(POINT_COST_LOW_LEVEL as i128, 0) {
        // For prod lets play it safe with a sane default value return
        // TODO: Consider if returning the Ultra level cost would be more fitting?
        HpFloat::new(POINT_COST_LOW_LEVEL as i128, 0)
    } else {
        tmp
    }
}

pub const SKILL_LEVEL_REGISTRY_OF_REGISTRIES: [&[&'static str]; 4] = [
    &SKILL_LEVEL_LOW_REGISTRY,
    &SKILL_LEVEL_MEDIUM_REGISTRY,
    &SKILL_LEVEL_HIGH_REGISTRY,
    &SKILL_LEVEL_ULTRA_REGISTRY,
];

pub const SKILL_LEVEL_TOTAL_AMOUNT: usize = SKILL_LEVEL_LOW_AMOUNT
    + SKILL_LEVEL_MEDIUM_AMOUNT
    + SKILL_LEVEL_HIGH_AMOUNT
    + SKILL_LEVEL_ULTRA_AMOUNT;

pub const SKILL_LEVEL_LOW_AMOUNT: usize = 10;
pub const SKILL_LEVEL_LOW_REGISTRY: [&'static str; SKILL_LEVEL_LOW_AMOUNT] = [
    "Unfamiliar",
    "Dabbling",
    "Weak Novice",
    "Novice",
    "Weak Beginner",
    "Beginner",
    "Capable Beginner",
    "Weak Amateur",
    "Amateur",
    "Capable Amateur",
];
pub fn random_low_skill_level(rng: &mut Fortuna) -> SSCCResult<(usize, &'static str)> {
    let ridx = rng
        .random_from_range(0, SKILL_LEVEL_LOW_AMOUNT)
        .map_err(|e| e.to_nemesis_error())?;
    Ok((ridx, SKILL_LEVEL_LOW_REGISTRY[ridx]))
}

pub const SKILL_LEVEL_MEDIUM_AMOUNT: usize = 10;
pub const SKILL_LEVEL_MEDIUM_REGISTRY: [&'static str; SKILL_LEVEL_MEDIUM_AMOUNT] = [
    "Weak Trainee",
    "Trainee",
    "Capable Trainee",
    "Weak Apprentice",
    "Apprentice",
    "Capable Apprentice",
    "Weak Professional",
    "Professional",
    "Capable Professional",
    "Skilled Professional",
];
pub fn random_medium_skill_level(rng: &mut Fortuna) -> SSCCResult<(usize, &'static str)> {
    let ridx = rng
        .random_from_range(0, SKILL_LEVEL_MEDIUM_AMOUNT)
        .map_err(|e| e.to_nemesis_error())?;
    Ok((ridx, SKILL_LEVEL_MEDIUM_REGISTRY[ridx]))
}

pub const SKILL_LEVEL_HIGH_AMOUNT: usize = 10;
pub const SKILL_LEVEL_HIGH_REGISTRY: [&'static str; SKILL_LEVEL_HIGH_AMOUNT] = [
    "Specialist",
    "Capable Specialist",
    "Skilled Specialist",
    "Expert",
    "Capable Expert",
    "Skilled Expert",
    "Master",
    "Skilled Master",
    "Industry-known Master",
    "Industry-leading Master",
];
pub fn random_high_skill_level(rng: &mut Fortuna) -> SSCCResult<(usize, &'static str)> {
    let ridx = rng
        .random_from_range(0, SKILL_LEVEL_HIGH_AMOUNT)
        .map_err(|e| e.to_nemesis_error())?;
    Ok((ridx, SKILL_LEVEL_HIGH_REGISTRY[ridx]))
}

pub const SKILL_LEVEL_ULTRA_AMOUNT: usize = 3;
pub const SKILL_LEVEL_ULTRA_REGISTRY: [&'static str; SKILL_LEVEL_ULTRA_AMOUNT] = [
    "Industry Legend",
    "Well-known Industry Legend",
    "Famous Industry Legend",
];
pub fn random_ultra_skill_level(rng: &mut Fortuna) -> SSCCResult<(usize, &'static str)> {
    let ridx = rng
        .random_from_range(0, SKILL_LEVEL_ULTRA_AMOUNT)
        .map_err(|e| e.to_nemesis_error())?;
    Ok((ridx, SKILL_LEVEL_ULTRA_REGISTRY[ridx]))
}
