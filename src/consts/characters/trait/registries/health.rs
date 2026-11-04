use crate::consts::characters::{
    base_values::{HEALTH, SOCIABILITY},
    flags::SPECIAL_NEEDS,
    r#trait::CharacterTrait,
};

static TRAIT_HEALTH_REGISTRY: [CharacterTrait; 15] = [
    CharacterTrait::new("Dwarf", [(SOCIABILITY.0, Some(-0.05))], []),
    CharacterTrait::new("Giant", [(SOCIABILITY.0, Some(0.1))], []),
    CharacterTrait::new("Scarred", [(SOCIABILITY.0, Some(-0.05))], []),
    CharacterTrait::new("Short", [(SOCIABILITY.0, Some(-0.05))], []),
    CharacterTrait::new("Puny", [(SOCIABILITY.0, Some(-0.05))], []),
    CharacterTrait::new("Mute", [(SOCIABILITY.0, Some(-5.0))], [SPECIAL_NEEDS]),
    CharacterTrait::new("Blind", [(SOCIABILITY.0, Some(-5.0))], [SPECIAL_NEEDS]),
    CharacterTrait::new("Deaf", [(SOCIABILITY.0, Some(-5.0))], [SPECIAL_NEEDS]),
    CharacterTrait::new("Paraplegic", [(SOCIABILITY.0, Some(-4.0))], [SPECIAL_NEEDS]),
    CharacterTrait::new("Tall", [(SOCIABILITY.0, Some(0.05))], []),
    CharacterTrait::new("Strong", [(SOCIABILITY.0, Some(0.25))], []),
    CharacterTrait::new("Spindly", [(HEALTH.0, Some(-0.5))], []),
    CharacterTrait::new("Herculean", [(SOCIABILITY.0, Some(0.5))], []),
    CharacterTrait::new("Feeble", [(HEALTH.0, Some(-0.5))], []),
    CharacterTrait::new("Disfigured", [], [SPECIAL_NEEDS]),
];
