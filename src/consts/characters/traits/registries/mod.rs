use crate::consts::{
    base_values::science::SCI_BONUS_ALL_FIELDS,
    characters::{
        flags::{ALWAYS_PROVIDES_RESEARCH, suicide::SUICIDE_NEVER_ON_SITE},
        traits::{
            CharacterTrait, TraitStatImpact,
            registries::{
                health::TRAIT_HEALTH_REGISTRY, intellect::TRAIT_INTELLECT_IQ_REGISTRY,
                looks::TRAIT_LOOKS_REGISTRY, mental_health::TRAIT_MENTAL_HEALTH_REGISTRY,
                social::TRAIT_SOCIAL_REGISTRY, work_ethics::TRAIT_WORK_ETHIC_REGISTRY,
            },
        },
    },
};

// TODO: Go through all traits and make their modifier values make sense
pub mod health;
pub mod intellect;
pub mod looks;
pub mod mental_health;
pub mod social;
pub mod work_ethics;

pub static TRAIT_REGISTRY_OF_REGISTRIES: [&[CharacterTrait]; 7] = [
    &TRAIT_INTELLECT_IQ_REGISTRY,
    &TRAIT_LOOKS_REGISTRY,
    &TRAIT_HEALTH_REGISTRY,
    &TRAIT_MENTAL_HEALTH_REGISTRY,
    &TRAIT_SOCIAL_REGISTRY,
    &TRAIT_WORK_ETHIC_REGISTRY,
    &TRAIT_NORMAL_REGISTRY,
];

pub fn get_trait_from_registry_by_name(trait_name: &str) -> Option<&'static CharacterTrait> {
    TRAIT_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .find(|entry| entry.name == trait_name)
}

const TRAIT_NORMAL_CURIOUS_NAME: &str = "Curious";
const TRAIT_NORMAL_INCURIOUS_NAME: &str = "Incurious";

pub static TRAIT_NORMAL_REGISTRY: [CharacterTrait; 23] = [
    CharacterTrait::new_normal(
        TRAIT_NORMAL_CURIOUS_NAME,
        [TraitStatImpact::new_flat((
            SCI_BONUS_ALL_FIELDS.0,
            Some(1.0),
        ))],
        [ALWAYS_PROVIDES_RESEARCH],
        [TRAIT_NORMAL_INCURIOUS_NAME],
    ),
    CharacterTrait::new_normal(
        TRAIT_NORMAL_INCURIOUS_NAME,
        [TraitStatImpact::new_flat((
            SCI_BONUS_ALL_FIELDS.0,
            Some(-1.0),
        ))],
        [],
        [TRAIT_NORMAL_CURIOUS_NAME],
    ),
    CharacterTrait::new_normal("Numberphile", [], [], []),
    CharacterTrait::new_normal("Strategist", [], [], []),
    CharacterTrait::new_normal("Tactician", [], [], []),
    CharacterTrait::new_normal("Pensive", [], [SUICIDE_NEVER_ON_SITE], []),
    CharacterTrait::new_normal("Erudite", [], [], []),
    // Placeholders / need to earn their place
    CharacterTrait::new_normal("Brooding", [], [], []),
    CharacterTrait::new_normal("Gluttonous", [], [], []),
    CharacterTrait::new_normal("Cynical", [], [], []),
    CharacterTrait::new_normal("Decadent", [], [], []),
    CharacterTrait::new_normal("Haughty", [], [], []),
    CharacterTrait::new_normal("Idolizer", [], [], []),
    CharacterTrait::new_normal("Playful", [], [], []),
    CharacterTrait::new_normal("Uncouth", [], [], []),
    CharacterTrait::new_normal("Willful", [], [], []),
    // Conflicting
    TRAIT_NORMAL_ARBITRARY,
    TRAIT_NORMAL_JUST,
    TRAIT_NORMAL_BRAVE,
    TRAIT_NORMAL_CRAVEN,
    TRAIT_NORMAL_TEMPERATE,
    TRAIT_NORMAL_PROUD,
    TRAIT_NORMAL_CALM,
];

const TRAIT_NORMAL_ARBITRARY_NAME: &str = "Arbitrary";
pub const TRAIT_NORMAL_ARBITRARY: CharacterTrait = CharacterTrait::new_normal(
    TRAIT_NORMAL_ARBITRARY_NAME,
    [],
    [],
    [TRAIT_NORMAL_JUST_NAME],
);
const TRAIT_NORMAL_JUST_NAME: &str = "Just";
pub const TRAIT_NORMAL_JUST: CharacterTrait = CharacterTrait::new_normal(
    TRAIT_NORMAL_JUST_NAME,
    [],
    [],
    [TRAIT_NORMAL_ARBITRARY_NAME],
);
const TRAIT_NORMAL_BRAVE_NAME: &str = "Brave";
pub const TRAIT_NORMAL_BRAVE: CharacterTrait =
    CharacterTrait::new_normal(TRAIT_NORMAL_BRAVE_NAME, [], [], [TRAIT_NORMAL_CRAVEN_NAME]);
const TRAIT_NORMAL_CRAVEN_NAME: &str = "Craven";
pub const TRAIT_NORMAL_CRAVEN: CharacterTrait =
    CharacterTrait::new_normal(TRAIT_NORMAL_CRAVEN_NAME, [], [], [TRAIT_NORMAL_BRAVE_NAME]);
const TRAIT_NORMAL_TEMPERATE_NAME: &str = "Temperate";
pub const TRAIT_NORMAL_TEMPERATE: CharacterTrait = CharacterTrait::new_normal(
    TRAIT_NORMAL_TEMPERATE_NAME,
    [],
    [],
    [TRAIT_NORMAL_PROUD_NAME],
);
const TRAIT_NORMAL_PROUD_NAME: &str = "Proud";
pub const TRAIT_NORMAL_PROUD: CharacterTrait = CharacterTrait::new_normal(
    TRAIT_NORMAL_PROUD_NAME,
    [],
    [],
    [TRAIT_NORMAL_TEMPERATE_NAME],
);
const TRAIT_NORMAL_ROWDY_NAME: &str = "Rowdy";
pub const TRAIT_NORMAL_ROWDY: CharacterTrait =
    CharacterTrait::new_normal(TRAIT_NORMAL_ROWDY_NAME, [], [], [TRAIT_NORMAL_CALM_NAME]);
const TRAIT_NORMAL_CALM_NAME: &str = "Calm";
pub const TRAIT_NORMAL_CALM: CharacterTrait =
    CharacterTrait::new_normal(TRAIT_NORMAL_CALM_NAME, [], [], [TRAIT_NORMAL_ROWDY_NAME]);

#[test]
fn all_conflicting_links_exist_and_are_symmetric() {
    for reg in TRAIT_REGISTRY_OF_REGISTRIES {
        for entry in reg {
            for &con in entry.conflicting_traits() {
                if let Some(target) = get_trait_from_registry_by_name(con) {
                    assert!(
                        target.conflicting_traits().contains(&entry.name),
                        "Trait '{}' conflicts with '{}', but '{}' does not list '{}' in its conflicting_traits",
                        entry.name,
                        con,
                        target.name,
                        entry.name
                    );
                } else {
                    assert!(
                        false,
                        "Trait '{}' conflicts with '{}', which does not exist in any registry",
                        entry.name, con
                    );
                };
            }
        }
    }
}
