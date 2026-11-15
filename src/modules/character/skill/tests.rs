use crate::{
    consts::characters::skills::registries::social::SKILL_SOCIAL_LYING,
    modules::character::skill::CharacterSkill,
};

#[test]
fn basics() {
    let mut skill = CharacterSkill::new(&SKILL_SOCIAL_LYING, &[]);
    let inital_level = skill.effective_level();
    skill.simulation_tick();
    assert!(skill.is_rusty());
    let rusted_level = skill.effective_level();
    assert!(inital_level == rusted_level);
    skill.level_up_immediately();
    let new_level = skill.effective_level();
    assert!(inital_level != new_level);
    assert!(!skill.is_rusty());
}
