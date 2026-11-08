use crate::consts::characters::{
    base_values::{HEALTH, SOCIABILITY},
    flags::SPECIAL_NEEDS,
    traits::{CharacterTrait, impact::TraitStatImpact},
};

define_trait_registry! {
    pub static TRAIT_HEALTH_REGISTRY = [
        TRAIT_HEALTH_DWARF => CharacterTrait::health("Dwarf")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -0.05)]),
        TRAIT_HEALTH_GIANT => CharacterTrait::health("Giant")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, 0.1)]),
        TRAIT_HEALTH_SCARRED => CharacterTrait::health("Scarred")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -0.05)]),
        TRAIT_HEALTH_SHORT => CharacterTrait::health("Short")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -0.05)]),
        TRAIT_HEALTH_PUNY => CharacterTrait::health("Puny")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -0.05)]),
        TRAIT_HEALTH_MUTE => CharacterTrait::health("Mute")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -5.0)])
            .with_flags([SPECIAL_NEEDS]),
        TRAIT_HEALTH_BLIND => CharacterTrait::health("Blind")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -5.0)])
            .with_flags([SPECIAL_NEEDS]),
        TRAIT_HEALTH_DEAF => CharacterTrait::health("Deaf")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -5.0)])
            .with_flags([SPECIAL_NEEDS]),
        TRAIT_HEALTH_PARAPLEGIC => CharacterTrait::health("Paraplegic")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, -4.0)])
            .with_flags([SPECIAL_NEEDS]),
        TRAIT_HEALTH_TALL => CharacterTrait::health("Tall")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, 0.05)]),
        TRAIT_HEALTH_STRONG => CharacterTrait::health("Strong")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, 0.25)]),
        TRAIT_HEALTH_SPINDLY => CharacterTrait::health("Spindly")
            .with_impacts([TraitStatImpact::new(HEALTH.0, -0.5)]),
        TRAIT_HEALTH_HERCULEAN => CharacterTrait::health("Herculean")
            .with_impacts([TraitStatImpact::new(SOCIABILITY.0, 0.5)]),
        TRAIT_HEALTH_FEEBLE => CharacterTrait::health("Feeble")
            .with_impacts([TraitStatImpact::new(HEALTH.0, -0.5)]),
        TRAIT_HEALTH_DISFIGURED => CharacterTrait::health("Disfigured")
            .with_flags([SPECIAL_NEEDS]),
    ];
}
