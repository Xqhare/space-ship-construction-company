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

pub mod health;
pub mod intellect;
pub mod looks;
pub mod mental_health;
pub mod social;
pub mod work_ethics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTraitRegistryId {
    Intellect(usize),
    Looks(usize),
    Health(usize),
    MentalHealth(usize),
    Social(usize),
    WorkEthics(usize),
    Normal(usize),
}

/// Attempt to get a trait via its id.
///
/// Returns None if the id is invalid.
pub fn get_trait_from_registry(
    trait_id: CharacterTraitRegistryId,
) -> Option<&'static CharacterTrait> {
    match trait_id {
        CharacterTraitRegistryId::Intellect(int) => {
            if TRAIT_INTELLECT_IQ_REGISTRY.len() > int {
                Some(&TRAIT_INTELLECT_IQ_REGISTRY[int])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::Looks(looks) => {
            if TRAIT_LOOKS_REGISTRY.len() > looks {
                Some(&TRAIT_LOOKS_REGISTRY[looks])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::Health(health) => {
            if TRAIT_HEALTH_REGISTRY.len() > health {
                Some(&TRAIT_HEALTH_REGISTRY[health])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::MentalHealth(mental) => {
            if TRAIT_MENTAL_HEALTH_REGISTRY.len() > mental {
                Some(&TRAIT_MENTAL_HEALTH_REGISTRY[mental])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::Social(soc) => {
            if TRAIT_SOCIAL_REGISTRY.len() > soc {
                Some(&TRAIT_SOCIAL_REGISTRY[soc])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::WorkEthics(weh) => {
            if TRAIT_WORK_ETHIC_REGISTRY.len() > weh {
                Some(&TRAIT_WORK_ETHIC_REGISTRY[weh])
            } else {
                None
            }
        }
        CharacterTraitRegistryId::Normal(norm) => {
            if TRAIT_NORMAL_REGISTRY.len() > norm {
                Some(&TRAIT_NORMAL_REGISTRY[norm])
            } else {
                None
            }
        }
    }
}

pub static TRAIT_NORMAL_REGISTRY: [CharacterTrait; 22] = [
    CharacterTrait::new(
        "Curious",
        [TraitStatImpact::new_flat((
            SCI_BONUS_ALL_FIELDS.0,
            Some(1.0),
        ))],
        [ALWAYS_PROVIDES_RESEARCH],
    ),
    CharacterTrait::new("Numberphile", [], []),
    CharacterTrait::new("Strategist", [], []),
    CharacterTrait::new("Tactician", [], []),
    CharacterTrait::new("Pensive", [], [SUICIDE_NEVER_ON_SITE]),
    CharacterTrait::new("Erudite", [], []),
    // Placeholders / need to earn their place
    CharacterTrait::new("Brooding", [], []),
    CharacterTrait::new("Gluttonous", [], []),
    CharacterTrait::new("Cynical", [], []),
    CharacterTrait::new("Decadent", [], []),
    CharacterTrait::new("Haughty", [], []),
    CharacterTrait::new("Idolizer", [], []),
    CharacterTrait::new("Playful", [], []),
    CharacterTrait::new("Uncouth", [], []),
    CharacterTrait::new("Willful", [], []),
    // Conflicting
    TRAIT_NORMAL_ARBITRARY,
    TRAIT_NORMAL_JUST,
    TRAIT_NORMAL_BRAVE,
    TRAIT_NORMAL_CRAVEN,
    TRAIT_NORMAL_TEMPERATE,
    TRAIT_NORMAL_PROUD,
    TRAIT_NORMAL_CALM,
];

// Scratch pad below

const TRAIT_NORMAL_ARBITRARY_NAME: &str = "Arbitrary";
pub const TRAIT_NORMAL_ARBITRARY: CharacterTrait =
    CharacterTrait::new(TRAIT_NORMAL_ARBITRARY_NAME, [], []);
const TRAIT_NORMAL_JUST_NAME: &str = "Just";
pub const TRAIT_NORMAL_JUST: CharacterTrait = CharacterTrait::new(TRAIT_NORMAL_JUST_NAME, [], []);
pub const TRAIT_NORMAL_BRAVE: CharacterTrait = CharacterTrait::new("Brave", [], []);
pub const TRAIT_NORMAL_CRAVEN: CharacterTrait = CharacterTrait::new("Craven", [], []);
pub const TRAIT_NORMAL_TEMPERATE: CharacterTrait = CharacterTrait::new("Temperate", [], []);
pub const TRAIT_NORMAL_PROUD: CharacterTrait = CharacterTrait::new("Proud", [], []);
pub const TRAIT_NORMAL_ROWDY: CharacterTrait = CharacterTrait::new("Rowdy", [], []);
pub const TRAIT_NORMAL_CALM: CharacterTrait = CharacterTrait::new("Calm", [], []);

// pub const fn get_conflicting_normal_traits(
//     char_trait: &CharacterTrait,
//     alt_trait_normal_conflicting_idx: (usize, usize),
// ) -> &'static [&'static CharacterTrait] {
//     let arb = ARBITRARY.as_bytes();
//     let just = JUST.as_bytes();
//     // Something like this would be nice
//     // match char_trait.name.as_bytes() {
//     //     // Sadly doesnt work:
//     //     //TRAIT_NORMAL_ARBITRARY.name.as_bytes() => {},
//     //     // And this way we are matching any with arb...
//     //     arb => &[&TRAIT_NORMAL_JUST],
//     //     just => &[&TRAIT_NORMAL_ARBITRARY],
//     //     _ => &[],
//     // }
//
//     // If the return were a single CharacterTrait this would work (but far far from ideal):
//     // *_idx is the index in the registry. First usize the index, second 0 or one
//     //TRAIT_NORMAL_CONFLICTING[alt_trait_normal_conflicting_idx.0].1
// }

// Fuck it, runtime check then
pub fn get_conflicting_normal_traits(char_trait: &CharacterTrait) -> Vec<&'static CharacterTrait> {
    // If more than 4 conflicting traits exists for any trait, increase the cap
    let mut out = Vec::with_capacity(4);
    match char_trait.name {
        TRAIT_NORMAL_ARBITRARY_NAME => out.push(&TRAIT_NORMAL_JUST),
        TRAIT_NORMAL_JUST_NAME => out.push(&TRAIT_NORMAL_ARBITRARY),
        _ => {}
    }
    out
}
