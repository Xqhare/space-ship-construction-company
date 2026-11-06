use crate::consts::characters::{
    base_values::{HEALTH, SOCIABILITY},
    flags::SPECIAL_NEEDS,
    traits::{CharacterTrait, TraitStatImpact},
};

pub static TRAIT_HEALTH_REGISTRY: [CharacterTrait; 15] = [
    CharacterTrait::new(
        "Dwarf",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
    ),
    CharacterTrait::new(
        "Giant",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.1)))],
        [],
    ),
    CharacterTrait::new(
        "Scarred",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
    ),
    CharacterTrait::new(
        "Short",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
    ),
    CharacterTrait::new(
        "Puny",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.05)))],
        [],
    ),
    CharacterTrait::new(
        "Mute",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
    ),
    CharacterTrait::new(
        "Blind",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
    ),
    CharacterTrait::new(
        "Deaf",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-5.0)))],
        [SPECIAL_NEEDS],
    ),
    CharacterTrait::new(
        "Paraplegic",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-4.0)))],
        [SPECIAL_NEEDS],
    ),
    CharacterTrait::new(
        "Tall",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.05)))],
        [],
    ),
    CharacterTrait::new(
        "Strong",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.25)))],
        [],
    ),
    CharacterTrait::new(
        "Spindly",
        [TraitStatImpact::new((HEALTH.0, Some(-0.5)))],
        [],
    ),
    CharacterTrait::new(
        "Herculean",
        [TraitStatImpact::new((SOCIABILITY.0, Some(0.5)))],
        [],
    ),
    CharacterTrait::new("Feeble", [TraitStatImpact::new((HEALTH.0, Some(-0.5)))], []),
    CharacterTrait::new("Disfigured", [], [SPECIAL_NEEDS]),
];
