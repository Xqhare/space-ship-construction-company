use crate::consts::characters::{
    flags::{ALWAYS_PROVIDES_ALL, ONLY_PROVIDES_NEEDED, suicide::SUICIDE_NEVER},
    traits::CharacterTrait,
};

pub static TRAIT_WORK_ETHIC_REGISTRY: [CharacterTrait; 8] = [
    CharacterTrait::new("Ambitious", [], [ALWAYS_PROVIDES_ALL]),
    CharacterTrait::new("Content", [], [SUICIDE_NEVER]),
    CharacterTrait::new("Diligent", [], [ALWAYS_PROVIDES_ALL]),
    CharacterTrait::new("Fickle", [], []),
    CharacterTrait::new("Impatient", [], []),
    CharacterTrait::new("Lazy", [], [ONLY_PROVIDES_NEEDED]),
    CharacterTrait::new("Patient", [], []),
    CharacterTrait::new("Stubborn", [], [SUICIDE_NEVER]),
];
