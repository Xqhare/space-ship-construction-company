use aequa::XffSerde;
use athena::{float::HpFloat, slot_map::SlotMap};
use horae::Utc;

use crate::{
    consts::characters::base_values::{ALL_CHAR_BASES, IQ},
    modules::character::Character,
    utils::{Utils, boxes::data_box::DataBox},
};

#[test]
fn basic() {
    let mut slot_map = SlotMap::new();
    let character_data_box = DataBox::new();
    let character_data_box_id = slot_map.insert(character_data_box);
    let utc_now = Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0);
    let mut utils =
        Utils::new("./data/", vec![1, 2, 3]).expect("Cannot create utils for testing character");
    let char = Character::new_random(
        &mut utils,
        character_data_box_id,
        slot_map
            .get_mut(character_data_box_id)
            .expect("Id must be valid"),
        &utc_now,
    );
    assert!(char.is_ok());
    let char = char.unwrap();
    let stats = char.get_full_stats(&mut slot_map);
    assert!(stats.is_ok());
    let stats = stats.unwrap();
    let iq = stats.get(IQ.0);
    assert!(iq.is_some());
    let iq = iq
        .unwrap()
        .as_hp_float()
        .expect("Value must always be HpFloat");
    assert!(iq <= &HpFloat::cast(149));
    assert!(iq >= &HpFloat::cast(50));
    for base in ALL_CHAR_BASES {
        assert!(stats.get(base.0).is_some())
    }
    assert!(char.birthday < utc_now);
    assert!(char.retirement > utc_now);
}

#[test]
fn xff_serde() {
    let mut slot_map = SlotMap::new();
    let character_data_box = DataBox::new();
    let character_data_box_id = slot_map.insert(character_data_box);
    let utc_now = Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0);
    let mut utils =
        Utils::new("./data/", vec![1, 2, 3]).expect("Cannot create utils for testing character");
    let char = Character::new_random(
        &mut utils,
        character_data_box_id,
        slot_map
            .get_mut(character_data_box_id)
            .expect("Id must be valid"),
        &utc_now,
    );
    assert!(char.is_ok());
    let char = char.unwrap();

    let xff = char.to_xff();
    // I am skipping the writing and reading to & from disk
    let from_xff = Character::from_xff(&xff);
    assert!(from_xff.is_ok());
    let from_xff = from_xff.unwrap();
    assert!(from_xff.name == char.name);
    assert!(from_xff.data_box == char.data_box);
    assert!(from_xff.birthday == char.birthday);
    assert!(from_xff.retirement == char.retirement);
    assert!(from_xff.teams == char.teams);
    println!("from: {:?}", from_xff.traits);
    println!();
    println!("char: {:?}", char.traits);
    assert!(from_xff.traits == char.traits);
    assert!(from_xff.skills == char.skills);
}
