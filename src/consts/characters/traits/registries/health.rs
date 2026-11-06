use crate::consts::characters::{
    base_values::{HEALTH, SOCIABILITY},
    flags::SPECIAL_NEEDS,
    traits::{CharacterTrait, TraitStatImpact},
};

pub static TRAIT_HEALTH_REGISTRY: [CharacterTrait; 15] = [
    CharacterTrait::new_health(
        "Dwarf",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Giant",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.1)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Scarred",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Short",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Puny",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Mute",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
        [],
    ),
    CharacterTrait::new_health(
        "Blind",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
        [],
    ),
    CharacterTrait::new_health(
        "Deaf",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
        [],
    ),
    CharacterTrait::new_health(
        "Paraplegic",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-4.0)))],
        [SPECIAL_NEEDS],
        [],
    ),
    CharacterTrait::new_health(
        "Tall",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.05)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Strong",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.25)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Spindly",
        [TraitStatImpact::new((HEALTH.0, Some(-0.5)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Herculean",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.5)))],
        [],
        [],
    ),
    CharacterTrait::new_health(
        "Feeble",
        [TraitStatImpact::new((HEALTH.0, Some(-0.5)))],
        [],
        [],
    ),
    CharacterTrait::new_health("Disfigured", [], [SPECIAL_NEEDS], []),
];
