use std::cmp::Ordering;

use athena::float::HpFloat;
use fortuna::Fortuna;
use tyche::RngApi;

use crate::error::SSCCResult;

#[derive(Debug, Clone, Copy)]
/// All inner `u8` values must be `< LEVEL_AMOUNT_*` for their tier.
///
/// Implements `PartialEq` and `PartialOrd`
pub enum Level {
    Low(u8),
    Medium(u8),
    High(u8),
    Ultra(u8),
}

impl Level {
    // NOTE: Performance will be not as good as a dedicated match (as in `level_up()`) and
    // flipping the call chain for level_up to call level_up_by(1)
    pub fn level_up_by(&self, amount: usize) -> Level {
        let mut tmp = *self;
        for _ in 0..amount {
            tmp = tmp.level_up()
        }
        tmp
    }
    // NOTE: Performance will be not as good as a dedicated match (as in `level_down()`) and
    // flipping the call chain for level_down to call level_down_by(1)
    pub fn level_down_by(&self, amount: usize) -> Level {
        let mut tmp = *self;
        for _ in 0..amount {
            tmp = tmp.level_down()
        }
        tmp
    }
    pub fn level_up(&self) -> Level {
        match self {
            Level::Low(level) => {
                let new_level = level + 1;
                if (new_level as usize) < SKILL_LEVEL_LOW_AMOUNT {
                    Level::Low(new_level)
                } else {
                    Level::Medium(0)
                }
            }
            Level::Medium(level) => {
                let new_level = level + 1;
                if (new_level as usize) < SKILL_LEVEL_MEDIUM_AMOUNT {
                    Level::Medium(new_level)
                } else {
                    Level::High(0)
                }
            }
            Level::High(level) => {
                let new_level = level + 1;
                if (new_level as usize) < SKILL_LEVEL_HIGH_AMOUNT {
                    Level::High(new_level)
                } else {
                    Level::Ultra(0)
                }
            }
            Level::Ultra(level) => {
                let new_level = level + 1;
                if (new_level as usize) < SKILL_LEVEL_LOW_AMOUNT {
                    Level::Ultra(new_level)
                } else {
                    *self
                }
            }
        }
    }
    pub fn level_down(&self) -> Level {
        match self {
            Level::Low(level) => {
                if *level == 0 {
                    *self
                } else {
                    Level::Low(level - 1)
                }
            }
            Level::Medium(level) => {
                if *level == 0 {
                    Level::Low(SKILL_LEVEL_LOW_AMOUNT as u8 - 1)
                } else {
                    Level::Medium(level - 1)
                }
            }
            Level::High(level) => {
                if *level == 0 {
                    Level::Medium(SKILL_LEVEL_MEDIUM_AMOUNT as u8 - 1)
                } else {
                    Level::High(level - 1)
                }
            }
            Level::Ultra(level) => {
                if *level == 0 {
                    Level::High(SKILL_LEVEL_HIGH_AMOUNT as u8 - 1)
                } else {
                    Level::Ultra(level - 1)
                }
            }
        }
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Level::Low(level) => match other {
                Level::Low(other_level) => level.eq(other_level),
                Level::Medium(_other_level) => false,
                Level::High(_other_level) => false,
                Level::Ultra(_other_level) => false,
            },
            Level::Medium(level) => match other {
                Level::Low(_other_level) => false,
                Level::Medium(other_level) => level.eq(other_level),
                Level::High(_other_level) => false,
                Level::Ultra(_other_level) => false,
            },
            Level::High(level) => match other {
                Level::Low(_other_level) => false,
                Level::Medium(_other_level) => false,
                Level::High(other_level) => level.eq(other_level),
                Level::Ultra(_other_level) => false,
            },
            Level::Ultra(level) => match other {
                Level::Low(_other_level) => false,
                Level::Medium(_other_level) => false,
                Level::High(_other_level) => false,
                Level::Ultra(other_level) => level.eq(other_level),
            },
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Level::Low(level) => match other {
                Level::Low(other_level) => level.partial_cmp(other_level),
                Level::Medium(_other_level) => Some(Ordering::Less),
                Level::High(_other_level) => Some(Ordering::Less),
                Level::Ultra(_other_level) => Some(Ordering::Less),
            },
            Level::Medium(level) => match other {
                Level::Low(_other_level) => Some(Ordering::Greater),
                Level::Medium(other_level) => level.partial_cmp(other_level),
                Level::High(_other_level) => Some(Ordering::Less),
                Level::Ultra(_other_level) => Some(Ordering::Less),
            },
            Level::High(level) => match other {
                Level::Low(_other_level) => Some(Ordering::Greater),
                Level::Medium(_other_level) => Some(Ordering::Greater),
                Level::High(other_level) => level.partial_cmp(other_level),
                Level::Ultra(_other_level) => Some(Ordering::Less),
            },
            Level::Ultra(level) => match other {
                Level::Low(_other_level) => Some(Ordering::Greater),
                Level::Medium(_other_level) => Some(Ordering::Greater),
                Level::High(_other_level) => Some(Ordering::Greater),
                Level::Ultra(other_level) => level.partial_cmp(other_level),
            },
        }
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
