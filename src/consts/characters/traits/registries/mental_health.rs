use crate::consts::characters::{flags::suicide::SUICIDE_NEVER_ON_SITE, traits::CharacterTrait};

define_trait_registry! {
    pub static TRAIT_MENTAL_HEALTH_REGISTRY = [
        TRAIT_MENTAL_HEALTH_PARANOID    => CharacterTrait::mental_health("Paranoid"),
        TRAIT_MENTAL_HEALTH_POSSESSED   => CharacterTrait::mental_health("Possessed"),
        TRAIT_MENTAL_HEALTH_MELANCHOLIC => CharacterTrait::mental_health("Melancholic")
            .with_flags([SUICIDE_NEVER_ON_SITE]),
        TRAIT_MENTAL_HEALTH_LUNATIC     => CharacterTrait::mental_health("Lunatic"),
    ];
}
