use crate::consts::characters::{
    flags::{ALWAYS_PROVIDES_ALL, ONLY_PROVIDES_NEEDED, suicide::SUICIDE_NEVER},
    traits::CharacterTrait,
};

define_trait_registry! {
    pub static TRAIT_WORK_ETHIC_REGISTRY = [
        TRAIT_WORK_ETHIC_AMBITIOUS => CharacterTrait::work_ethic("Ambitious")
            .with_flags([ALWAYS_PROVIDES_ALL]),
        TRAIT_WORK_ETHIC_CONTENT   => CharacterTrait::work_ethic("Content")
            .with_flags([SUICIDE_NEVER]),
        TRAIT_WORK_ETHIC_DILIGENT  => CharacterTrait::work_ethic("Diligent")
            .with_flags([ALWAYS_PROVIDES_ALL]),
        TRAIT_WORK_ETHIC_FICKLE    => CharacterTrait::work_ethic("Fickle"),
        TRAIT_WORK_ETHIC_IMPATIENT => CharacterTrait::work_ethic("Impatient"),
        TRAIT_WORK_ETHIC_LAZY      => CharacterTrait::work_ethic("Lazy")
            .with_flags([ONLY_PROVIDES_NEEDED]),
        TRAIT_WORK_ETHIC_PATIENT   => CharacterTrait::work_ethic("Patient"),
        TRAIT_WORK_ETHIC_STUBBORN  => CharacterTrait::work_ethic("Stubborn")
            .with_flags([SUICIDE_NEVER]),
    ];
}
