use std::time::Duration;

use athena::float::HpFloat;
use horae::Utc;
use tyche::RngApi;

use crate::{
    consts::characters::{
        RETIREMENT_AGE,
        skills::{ConstSkill, registries::SKILL_REGISTRY_OF_REGISTRIES},
        traits::{
            CharacterTrait,
            registries::{
                intellect::{
                    TRAIT_INTELLECT_ASTUTE, TRAIT_INTELLECT_BRIGHT, TRAIT_INTELLECT_DENSE,
                    TRAIT_INTELLECT_DIM, TRAIT_INTELLECT_DIMWIT, TRAIT_INTELLECT_DULL,
                    TRAIT_INTELLECT_GENIUS, TRAIT_INTELLECT_SHREWD, TRAIT_INTELLECT_SIMPLETON,
                    TRAIT_INTELLECT_SMART,
                },
                looks::{
                    TRAIT_LOOKS_ABERRANT, TRAIT_LOOKS_APPEALING, TRAIT_LOOKS_ATTRACTIVE,
                    TRAIT_LOOKS_BEAUTIFUL, TRAIT_LOOKS_GORGEOUS, TRAIT_LOOKS_HIDEOUS,
                    TRAIT_LOOKS_PRETTY, TRAIT_LOOKS_UNSIGHTLY,
                },
            },
        },
    },
    error::SSCCResult,
    utils::{
        Utils,
        rng::{rand_bool, random_from_range_inclusive},
    },
};

/// Based on a normalized & capped Terman & Merrill (1960) distribution:
///
/// | # | Trait | IQ Range | Probability | Terman & Merrill Ref |
/// | --- | --- | --- | --- | --- |
/// | 1 | **Genius** | 140–149 | `1.5625%` | 1.1% (140–169: 1.33%) |
/// | 2 | **Astute** | 130–139 | `3.125%` | 3.1% |
/// | 3 | **Shrewd** | 120–129 | `6.25%` | 8.2% |
/// | 4 | **Smart** | 110–119 | `12.5%` | 18.1% |
/// | 5 | **Bright** | 100–109 | `26.5625%` (`25% + 1.5625%`) | 23.5% |
/// | 6 | **Dim** | 90–99 | `26.5625%` (`25% + 1.5625%`) | 23.0% |
/// | 7 | **Dull** | 80–89 | `12.5%` | 14.5% |
/// | 8 | **Dense** | 70–79 | `6.25%` | 5.6% |
/// | 9 | **Dimwit** | 60–69 | `3.125%` | 2.0% |
/// | 10 | **Simpleton** | 40–59 | `1.5625%` | 0.63% |
pub fn new_random_char_iq(utils: &mut Utils) -> SSCCResult<(HpFloat, &'static CharacterTrait)> {
    if rand_bool(utils)? {
        if rand_bool(utils)? {
            let iq = random_from_range_inclusive(utils, 100, 109)?;
            Ok((iq, &TRAIT_INTELLECT_BRIGHT))
        } else {
            let iq = random_from_range_inclusive(utils, 90, 99)?;
            Ok((iq, &TRAIT_INTELLECT_DIM))
        }
    } else {
        if rand_bool(utils)? {
            if rand_bool(utils)? {
                let iq = random_from_range_inclusive(utils, 110, 119)?;
                Ok((iq, &TRAIT_INTELLECT_SMART))
            } else {
                let iq = random_from_range_inclusive(utils, 80, 89)?;
                Ok((iq, &TRAIT_INTELLECT_DULL))
            }
        } else {
            if rand_bool(utils)? {
                if rand_bool(utils)? {
                    let iq = random_from_range_inclusive(utils, 120, 129)?;
                    Ok((iq, &TRAIT_INTELLECT_SHREWD))
                } else {
                    let iq = random_from_range_inclusive(utils, 70, 79)?;
                    Ok((iq, &TRAIT_INTELLECT_DENSE))
                }
            } else {
                if rand_bool(utils)? {
                    if rand_bool(utils)? {
                        let iq = random_from_range_inclusive(utils, 130, 139)?;
                        Ok((iq, &TRAIT_INTELLECT_ASTUTE))
                    } else {
                        let iq = random_from_range_inclusive(utils, 60, 69)?;
                        Ok((iq, &TRAIT_INTELLECT_DIMWIT))
                    }
                } else {
                    if rand_bool(utils)? {
                        if rand_bool(utils)? {
                            let iq = random_from_range_inclusive(utils, 140, 149)?;
                            Ok((iq, &TRAIT_INTELLECT_GENIUS))
                        } else {
                            let iq = random_from_range_inclusive(utils, 50, 59)?;
                            Ok((iq, &TRAIT_INTELLECT_SIMPLETON))
                        }
                    } else {
                        if rand_bool(utils)? {
                            let iq = random_from_range_inclusive(utils, 100, 109)?;
                            Ok((iq, &TRAIT_INTELLECT_BRIGHT))
                        } else {
                            let iq = random_from_range_inclusive(utils, 90, 99)?;
                            Ok((iq, &TRAIT_INTELLECT_DIM))
                        }
                    }
                }
            }
        }
    }
}
/// Based on nothing but feeling:
///
/// - **Gorgeous:** `3.125%`
/// - **Beautiful:** `6.25%`
/// - **Attractive:** `12.5%`
/// - **Pretty:** `25%`
/// - **Appealing:** `25%`
/// - **Unsightly:** `12.5%`
/// - **Hideous:** `6.25%`
/// - **Aberrant:** `3.125%`
pub fn new_random_char_look(utils: &mut Utils) -> SSCCResult<&'static CharacterTrait> {
    if rand_bool(utils)? {
        if rand_bool(utils)? {
            Ok(&TRAIT_LOOKS_PRETTY)
        } else {
            Ok(&TRAIT_LOOKS_APPEALING)
        }
    } else {
        if rand_bool(utils)? {
            if rand_bool(utils)? {
                Ok(&TRAIT_LOOKS_ATTRACTIVE)
            } else {
                Ok(&TRAIT_LOOKS_UNSIGHTLY)
            }
        } else {
            if rand_bool(utils)? {
                if rand_bool(utils)? {
                    Ok(&TRAIT_LOOKS_BEAUTIFUL)
                } else {
                    Ok(&TRAIT_LOOKS_HIDEOUS)
                }
            } else {
                if rand_bool(utils)? {
                    Ok(&TRAIT_LOOKS_GORGEOUS)
                } else {
                    Ok(&TRAIT_LOOKS_ABERRANT)
                }
            }
        }
    }
}

pub fn new_random_char_birthday(utils: &mut Utils, utc_now: &Utc) -> SSCCResult<Utc> {
    let ceiling = {
        if rand_bool(utils)? {
            RETIREMENT_AGE / 3
        } else {
            if rand_bool(utils)? {
                RETIREMENT_AGE / 2
            } else {
                if rand_bool(utils)? {
                    RETIREMENT_AGE / 2
                } else {
                    RETIREMENT_AGE
                }
            }
        }
    };
    let age = utils
        .rng
        .random_from_range(18, ceiling)
        .map_err(|err| err.to_nemesis_error())? as u16;
    let year = utc_now.date().year - age;
    let month = utils
        .rng
        .random_from_range_inclusive(1, 12)
        .map_err(|err| err.to_nemesis_error())? as u8;
    // Because of leap year math & 30 vs 31 day months: I simplify with a random duration
    // Bonus: The time of birth is also random to the second
    let days_dur_in_h = Duration::from_secs(
        utils
            .rng
            .random_from_range_inclusive(1, 31 * 24 * 60 * 60)
            .map_err(|err| err.to_nemesis_error())? as u64,
    );
    Ok(Utc::from_ymd_hms(year, month, 1, 0, 0, 0) + days_dur_in_h)
}
pub fn new_random_char_skills(
    utils: &mut Utils,
    age_in_years: HpFloat,
) -> SSCCResult<Vec<&'static ConstSkill>> {
    let inital_skill_amount: HpFloat = HpFloat::cast(10);
    let max_skills: HpFloat = inital_skill_amount + (inital_skill_amount * (age_in_years / 100.0));
    debug_assert!(
        max_skills.is_positive(),
        "Maximum skill amount is negative!"
    );
    let max_skills = if max_skills.is_negative() {
        let tmp: HpFloat = max_skills * -1.0;
        tmp.get_integer_part() as usize
    } else {
        max_skills.get_integer_part() as usize
    };
    let mut tmp: Vec<&'static ConstSkill> = Vec::with_capacity(max_skills);
    for _ in 0..utils
        .rng
        .random_from_range_inclusive(1, max_skills)
        .map_err(|err| err.to_nemesis_error())?
    {
        let base = &SKILL_REGISTRY_OF_REGISTRIES[utils
            .rng
            .random_index(SKILL_REGISTRY_OF_REGISTRIES.len())
            .map_err(|err| err.to_nemesis_error())?];
        let skill: &ConstSkill = base[utils
            .rng
            .random_index(base.len())
            .map_err(|err| err.to_nemesis_error())?];
        tmp.push(skill)
    }
    Ok(tmp)
}
