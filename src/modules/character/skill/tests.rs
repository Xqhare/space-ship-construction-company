use aequa::XffSerde;

use crate::{
    consts::characters::{
        skills::{levels::Level, registries::social::SKILL_SOCIAL_LYING},
        traits::registries::intellect::TRAIT_INTELLECT_GENIUS,
    },
    modules::character::skill::CharacterSkill,
};

#[test]
fn basics() {
    let mut skill = CharacterSkill::new(&SKILL_SOCIAL_LYING, &[]);
    let inital_level = skill.effective_level();
    skill.simulation_tick();
    let rusted_level = skill.effective_level();
    assert!(inital_level == rusted_level);
    skill.level_up_immediately();
    let new_level = skill.effective_level();
    assert!(inital_level != new_level);
    assert!(!skill.is_rusty());
}

#[test]
fn rust_and_level_progression() {
    let mut skill = CharacterSkill::new(&SKILL_SOCIAL_LYING, &[&TRAIT_INTELLECT_GENIUS]);
    assert_eq!(*skill.level(), Level::Low(0));

    // Level up to Low(2) so rust has room to lower effective_level
    skill.level_up_immediately();
    skill.level_up_immediately();
    assert_eq!(*skill.level(), Level::Low(2));
    assert_eq!(skill.effective_level(), Level::Low(2));

    // 5 ticks: not rusty yet; 6th tick: 1 stack of rust
    for _ in 0..5 {
        skill.simulation_tick();
        assert!(!skill.is_rusty());
    }
    skill.simulation_tick();
    assert!(skill.is_rusty());
    assert_eq!(*skill.level(), Level::Low(2)); // earned level preserved
    assert_eq!(skill.effective_level(), Level::Low(1)); // effective level reduced by 1

    // Accumulate a 2nd rust stack
    for _ in 0..6 {
        skill.simulation_tick();
    }
    assert_eq!(skill.effective_level(), Level::Low(0));

    // Using the skill twice in the same sim tick only removes 1 rust stack
    skill.skill_used();
    skill.skill_used();
    assert_eq!(skill.effective_level(), Level::Low(1));

    // Next sim tick resets the per-tick lock so use removes the last stack
    skill.simulation_tick();
    skill.skill_used();
    assert!(!skill.is_rusty());
    assert_eq!(skill.effective_level(), Level::Low(2));
}

#[test]
fn skill_xff_roundtrip() {
    let mut skill = CharacterSkill::new(&SKILL_SOCIAL_LYING, &[&TRAIT_INTELLECT_GENIUS]);
    skill.set_level(Level::Medium(3));
    for _ in 0..6 {
        skill.simulation_tick();
    }
    let restored = CharacterSkill::from_xff(&skill.to_xff()).unwrap();
    assert_eq!(restored, skill);
}
