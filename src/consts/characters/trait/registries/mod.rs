use crate::consts::{
    base_values::science::SCI_BONUS_ALL_FIELDS,
    characters::{
        base_values::{IQ, SOCIABILITY},
        flags::{
            ALWAYS_PROVIDES_ALL, ALWAYS_PROVIDES_RESEARCH, ONLY_PROVIDES_NEEDED,
            suicide::{SUICIDE_NEVER, SUICIDE_NEVER_OFF_SITE, SUICIDE_NEVER_ON_SITE},
        },
        r#trait::CharacterTrait,
    },
};

pub mod health;
pub mod intellect;

static TRAIT_MENTAL_HEALTH_REGISTRY: [CharacterTrait; 4] = [
    CharacterTrait::new("Paranoid", [], []),
    CharacterTrait::new("Possessed", [], []),
    CharacterTrait::new("Melancholic", [], [SUICIDE_NEVER_ON_SITE]),
    CharacterTrait::new("Lunatic", [], []),
];

static TRAIT_SOCIAL_REGISTRY: [CharacterTrait; 22] = [
    CharacterTrait::new("Booming Voice", [], []),
    CharacterTrait::new(
        "Machiavellian",
        [],
        [SUICIDE_NEVER_OFF_SITE, ONLY_PROVIDES_NEEDED],
    ),
    CharacterTrait::new("Silver Tongue", [], []),
    CharacterTrait::new("Conciliatory", [], []),
    CharacterTrait::new("Timid", [], []),
    CharacterTrait::new("Groomed", [], []),
    CharacterTrait::new("Fussy", [], []),
    CharacterTrait::new("Affectionate", [], []),
    CharacterTrait::new("Wrathful", [], []),
    CharacterTrait::new("Vengeful", [], []),
    CharacterTrait::new("Trusting", [], []),
    CharacterTrait::new("Shy", [], [SUICIDE_NEVER_ON_SITE]),
    CharacterTrait::new("Sadistic", [], []),
    CharacterTrait::new("Honest", [], []),
    CharacterTrait::new("Gregarious", [], []),
    CharacterTrait::new("Generous", [], []),
    CharacterTrait::new("Forgiving", [], []),
    CharacterTrait::new("Deceitful", [], []),
    CharacterTrait::new("Compassionate", [], [SUICIDE_NEVER]),
    CharacterTrait::new("Charming", [], []),
    CharacterTrait::new("Bossy", [], []),
    CharacterTrait::new("Callous", [], []),
];

static TRAIT_LOOKS_REGISTRY: [CharacterTrait; 8] = [
    CharacterTrait::new("Aberrant", [], []),   // 3.125
    CharacterTrait::new("Hideous", [], []),    // 6.25
    CharacterTrait::new("Unsightly", [], []),  // 12.5
    CharacterTrait::new("Appealing", [], []),  // 25
    CharacterTrait::new("Pretty", [], []),     // 25
    CharacterTrait::new("Attractive", [], []), // 12.5
    CharacterTrait::new("Beautiful", [], []),  // 6.25
    CharacterTrait::new("Gorgeous", [], []),   // 3.125
];
pub type CharacterTraitRegistryId = u8;

static TRAIT_NORMAL_REGISTRY: [CharacterTrait; 39] = [
    // Ck3 Tratis
    CharacterTrait::new("Ambitious", [], [ALWAYS_PROVIDES_ALL]),
    CharacterTrait::new("Arbitrary", [], []),
    CharacterTrait::new("Arrogant", [], []),
    CharacterTrait::new("Brave", [], []),
    CharacterTrait::new("Calm", [], []),
    CharacterTrait::new("Content", [], [SUICIDE_NEVER]),
    CharacterTrait::new("Craven", [], []),
    CharacterTrait::new(
        "Curious",
        [(IQ.0, Some(10.0)), (SCI_BONUS_ALL_FIELDS.0, Some(1.0))],
        [ALWAYS_PROVIDES_RESEARCH],
    ),
    CharacterTrait::new("Cynical", [], []),
    CharacterTrait::new("Diligent", [], [ALWAYS_PROVIDES_ALL]),
    CharacterTrait::new("Fickle", [], []),
    CharacterTrait::new("Gluttonous", [], []),
    CharacterTrait::new("Greedy", [], []),
    CharacterTrait::new("Humble", [], []),
    CharacterTrait::new("Impatient", [], []),
    CharacterTrait::new("Just", [], []),
    CharacterTrait::new("Lazy", [], [ONLY_PROVIDES_NEEDED]),
    CharacterTrait::new("Patient", [], []),
    CharacterTrait::new("Pensive", [], [SUICIDE_NEVER_ON_SITE]),
    CharacterTrait::new("Rowdy", [], []),
    CharacterTrait::new("Stubborn", [], [SUICIDE_NEVER]),
    CharacterTrait::new("Temperate", [], []),
    // Ck2 Traits
    CharacterTrait::new("Brooding", [], []),
    CharacterTrait::new("Charitable", [], []),
    CharacterTrait::new("Conscientious", [], []),
    CharacterTrait::new("Decadent", [], []),
    CharacterTrait::new("Erudite", [], []),
    CharacterTrait::new("Haughty", [], []),
    CharacterTrait::new("Idolizer", [], []),
    CharacterTrait::new("Indolent", [], []),
    CharacterTrait::new("Kind", [], []),
    CharacterTrait::new("Playful", [], []),
    CharacterTrait::new("Proud", [], []),
    CharacterTrait::new("Uncouth", [], []),
    CharacterTrait::new("Willful", [], []),
    CharacterTrait::new("Wroth", [], []),
    // Ck 2 Rolls Community Mod Traits
    CharacterTrait::new("Numberphile", [], []),
    CharacterTrait::new("Strategist", [], []),
    CharacterTrait::new("Tactician", [], []),
];
