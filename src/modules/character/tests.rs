use aequa::XffSerde;
use athena::{float::HpFloat, slot_map::SlotMap};
use horae::Utc;

use crate::{
    consts::characters::{
        base_values::{ALL_CHAR_BASES, IQ, STRENGTH},
        flags::{ALWAYS_PROVIDES_RESEARCH, SPECIAL_NEEDS},
        skills::{
            levels::Level,
            registries::social::{SKILL_SOCIAL_JUDGE_OF_INTENT, SKILL_SOCIAL_LYING},
        },
        traits::registries::{
            TRAIT_NORMAL_CURIOUS, TRAIT_NORMAL_INCURIOUS,
            health::TRAIT_HEALTH_MUTE,
            intellect::{TRAIT_INTELLECT_BRIGHT, TRAIT_INTELLECT_DIMWIT, TRAIT_INTELLECT_GENIUS},
            looks::TRAIT_LOOKS_PRETTY,
            mental_health::TRAIT_MENTAL_HEALTH_MELANCHOLIC,
            social::TRAIT_SOCIAL_MACHIAVELLIAN,
            work_ethics::TRAIT_WORK_ETHIC_DILIGENT,
        },
    },
    modules::character::{Character, name::CharacterName, r#trait::CharacterTraitStore},
    utils::{Utils, boxes::data_box::DataBox},
};

#[test]
fn basic() {
    let mut slot_map = SlotMap::new();
    let utc_now = Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0);
    let mut utils =
        Utils::new("./data/", vec![1, 2, 3]).expect("Cannot create utils for testing character");

    for _ in 0..50 {
        let character_data_box_id = slot_map.insert(DataBox::new());
        let char = Character::new_random(
            &mut utils,
            character_data_box_id,
            slot_map
                .get_mut(character_data_box_id)
                .expect("Id must be valid"),
            &utc_now,
        )
        .expect("Random character generation must succeed");

        let stats = char
            .get_full_stats(&slot_map)
            .expect("get_full_stats must succeed");
        let iq = stats
            .get(IQ.0)
            .expect("IQ must exist")
            .as_hp_float()
            .expect("Value must always be HpFloat");
        assert!(iq <= &HpFloat::cast(149));
        assert!(iq >= &HpFloat::cast(50));

        for base in ALL_CHAR_BASES {
            assert!(stats.get(base.0).is_some());
        }
        assert!(char.birthday < utc_now);
        assert!(char.retirement > utc_now);
        assert!(!char.skills.is_empty());
    }
}

#[test]
fn ultra_skill_works() {
    let mut slot_map = SlotMap::new();
    let utc_now = Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0);
    let mut utils =
        Utils::new("./data/", vec![1, 2, 3]).expect("Cannot create utils for testing character");
    let ultra_box_id = slot_map.insert(DataBox::new());
    let ultra_char = Character::new_random_one_ultra_skill(
        &mut utils,
        ultra_box_id,
        slot_map.get_mut(ultra_box_id).expect("Id must be valid"),
        &utc_now,
    )
    .expect("Ultra skill character generation must succeed");
    assert!(
        ultra_char
            .skills
            .iter()
            .any(|s| matches!(s.level(), Level::Ultra(_)))
    );
}

#[test]
fn xff_serde() {
    let mut slot_map = SlotMap::new();
    let character_data_box = DataBox::new();
    let character_data_box_id = slot_map.insert(character_data_box);
    let utc_now = Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0);
    let mut utils =
        Utils::new("./data/", vec![1, 2, 3]).expect("Cannot create utils for testing character");
    let mut char = Character::new_random(
        &mut utils,
        character_data_box_id,
        slot_map
            .get_mut(character_data_box_id)
            .expect("Id must be valid"),
        &utc_now,
    )
    .expect("Character creation must succeed");

    char.force_trait(&TRAIT_HEALTH_MUTE, &mut slot_map);
    char.force_trait(&TRAIT_MENTAL_HEALTH_MELANCHOLIC, &mut slot_map);
    char.force_trait(&TRAIT_SOCIAL_MACHIAVELLIAN, &mut slot_map);
    char.force_trait(&TRAIT_WORK_ETHIC_DILIGENT, &mut slot_map);
    char.force_trait(&TRAIT_NORMAL_CURIOUS, &mut slot_map);

    for _ in 0..6 {
        char.tick_skills_rust();
    }
    assert!(char.skills.iter().all(|s| s.is_rusty()));

    let xff = char.to_xff();
    let from_xff = Character::from_xff(&xff).expect("Character::from_xff must succeed");
    assert_eq!(from_xff.name, char.name);
    assert_eq!(from_xff.data_box, char.data_box);
    assert_eq!(from_xff.birthday, char.birthday);
    assert_eq!(from_xff.retirement, char.retirement);
    assert_eq!(from_xff.teams, char.teams);
    assert_eq!(from_xff.traits, char.traits);
    assert_eq!(from_xff.skills, char.skills);
}

#[test]
fn traits_and_skill_use_lifecycle() {
    let mut slot_map = SlotMap::new();
    let mut data_box = DataBox::new();
    data_box.stats.add_base_value(IQ.0, &HpFloat::cast(105));
    data_box
        .stats
        .add_base_value(STRENGTH.0, &HpFloat::cast(500));
    let data_box_id = slot_map.insert(data_box);

    let birthday = Utc::from_ymd_hms(2470, 1, 1, 0, 0, 0);
    let traits = CharacterTraitStore {
        intellect: &TRAIT_INTELLECT_BRIGHT,
        looks: &TRAIT_LOOKS_PRETTY,
        health: None,
        mental_health: None,
        social: None,
        work_ethic: None,
        normal_traits: Vec::new(),
    };
    let name = CharacterName::new_custom("Jane Doe", "Jane", None, "Doe", "JD");

    let mut char = Character::new_custom(
        data_box_id,
        traits,
        Vec::new(),
        Vec::new(),
        birthday,
        name,
        slot_map.get_mut(data_box_id).expect("Id must be valid"),
    )
    .expect("Custom character creation must succeed");

    // 1. Learning an unknown skill via tick_skill_use starts it at Level::Low(0)
    char.tick_skill_use(&mut slot_map, None, Some(&[SKILL_SOCIAL_LYING.name]))
        .expect("tick_skill_use must succeed");
    assert_eq!(char.skills.len(), 1);
    assert_eq!(*char.skills[0].level(), Level::Low(0));

    // 2. Forcing Genius increases skill affinity; forcing Dimwit lowers it
    let base_affinity = *char.skills[0].affinity();
    char.force_trait(&TRAIT_INTELLECT_GENIUS, &mut slot_map);
    let genius_affinity = *char.skills[0].affinity();
    assert!(genius_affinity > base_affinity);

    char.force_trait(&TRAIT_INTELLECT_DIMWIT, &mut slot_map);
    assert!(*char.skills[0].affinity() < base_affinity);

    // Restore Genius so leveling up via tick_skill_use is fast
    char.force_trait(&TRAIT_INTELLECT_GENIUS, &mut slot_map);

    // 3. Repeatedly using the skill levels it up above Low(0)
    for _ in 0..40 {
        char.tick_skill_use(&mut slot_map, None, Some(&[SKILL_SOCIAL_LYING.name]))
            .expect("tick_skill_use must succeed");
    }
    let earned_level = *char.skills[0].level();
    assert!(earned_level > Level::Low(0));

    // 4. 6 idle simulation ticks accumulate 1 stack of rust, lowering effective_level
    for _ in 0..6 {
        char.tick_skills_rust();
    }
    assert!(char.skills[0].is_rusty());
    assert_eq!(*char.skills[0].level(), earned_level);
    assert_eq!(
        char.skills[0].effective_level(),
        earned_level.level_down_by(1)
    );

    // Using the skill removes the rust stack
    char.tick_skill_use(&mut slot_map, None, Some(&[SKILL_SOCIAL_LYING.name]))
        .expect("tick_skill_use must succeed");
    assert!(!char.skills[0].is_rusty());

    // 5. Adding, forcing conflicting, and removing traits syncs DataBox flags
    char.add_trait(&TRAIT_NORMAL_CURIOUS, &mut slot_map);
    assert!(char.traits.has_trait(TRAIT_NORMAL_CURIOUS.name));
    assert!(
        char.data_box_flag_box(&mut slot_map)
            .unwrap()
            .has_flag(ALWAYS_PROVIDES_RESEARCH)
    );

    // Forcing conflicting trait (Incurious) removes Curious and its flag
    char.force_trait(&TRAIT_NORMAL_INCURIOUS, &mut slot_map);
    assert!(!char.traits.has_trait(TRAIT_NORMAL_CURIOUS.name));
    assert!(char.traits.has_trait(TRAIT_NORMAL_INCURIOUS.name));
    assert!(
        !char
            .data_box_flag_box(&mut slot_map)
            .unwrap()
            .has_flag(ALWAYS_PROVIDES_RESEARCH)
    );

    // Adding & removing a health trait syncs its SPECIAL_NEEDS flag
    char.add_trait(&TRAIT_HEALTH_MUTE, &mut slot_map);
    assert!(
        char.data_box_flag_box(&mut slot_map)
            .unwrap()
            .has_flag(SPECIAL_NEEDS)
    );
    char.remove_trait(&TRAIT_HEALTH_MUTE, &mut slot_map);
    assert!(
        !char
            .data_box_flag_box(&mut slot_map)
            .unwrap()
            .has_flag(SPECIAL_NEEDS)
    );

    // Adding a duplicate skill is a no-op
    char.add_skill_at_low_zero_level(
        &SKILL_SOCIAL_JUDGE_OF_INTENT,
        slot_map.get_mut(data_box_id).unwrap(),
    )
    .unwrap();
    char.add_skill_at_low_zero_level(
        &SKILL_SOCIAL_JUDGE_OF_INTENT,
        slot_map.get_mut(data_box_id).unwrap(),
    )
    .unwrap();
    assert_eq!(char.skills.len(), 2);
}
