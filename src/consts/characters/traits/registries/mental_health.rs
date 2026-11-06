use crate::consts::characters::{flags::suicide::SUICIDE_NEVER_ON_SITE, traits::CharacterTrait};

pub static TRAIT_MENTAL_HEALTH_REGISTRY: [CharacterTrait; 4] = [
    CharacterTrait::new("Paranoid", [], []),
    CharacterTrait::new("Possessed", [], []),
    CharacterTrait::new("Melancholic", [], [SUICIDE_NEVER_ON_SITE]),
    CharacterTrait::new("Lunatic", [], []),
];
