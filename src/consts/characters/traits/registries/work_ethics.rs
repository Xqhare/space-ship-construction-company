use crate::consts::characters::{
    flags::{ALWAYS_PROVIDES_ALL, ONLY_PROVIDES_NEEDED, suicide::SUICIDE_NEVER},
    traits::CharacterTrait,
};

pub static TRAIT_WORK_ETHIC_REGISTRY: [CharacterTrait; 8] = [
    CharacterTrait::new_work_ethic("Ambitious", [], [ALWAYS_PROVIDES_ALL], []),
    CharacterTrait::new_work_ethic("Content", [], [SUICIDE_NEVER], []),
    CharacterTrait::new_work_ethic("Diligent", [], [ALWAYS_PROVIDES_ALL], []),
    CharacterTrait::new_work_ethic("Fickle", [], [], []),
    CharacterTrait::new_work_ethic("Impatient", [], [], []),
    CharacterTrait::new_work_ethic("Lazy", [], [ONLY_PROVIDES_NEEDED], []),
    CharacterTrait::new_work_ethic("Patient", [], [], []),
    CharacterTrait::new_work_ethic("Stubborn", [], [SUICIDE_NEVER], []),
];
