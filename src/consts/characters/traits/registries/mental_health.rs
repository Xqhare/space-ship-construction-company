use crate::consts::characters::{flags::suicide::SUICIDE_NEVER_ON_SITE, traits::CharacterTrait};

pub static TRAIT_MENTAL_HEALTH_REGISTRY: [CharacterTrait; 4] = [
    CharacterTrait::new_mental_health("Paranoid", [], [], []),
    CharacterTrait::new_mental_health("Possessed", [], [], []),
    CharacterTrait::new_mental_health("Melancholic", [], [SUICIDE_NEVER_ON_SITE], []),
    CharacterTrait::new_mental_health("Lunatic", [], [], []),
];
