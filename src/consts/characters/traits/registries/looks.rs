use crate::consts::characters::{
    base_values::SOCIABILITY,
    traits::{CharacterTrait, TraitStatImpact},
};

pub static TRAIT_LOOKS_REGISTRY: [CharacterTrait; 8] = [
    CharacterTrait::new_looks(
        "Aberrant",
        [TraitStatImpact::new((SOCIABILITY.0, Some(-0.1)))],
        [],
        [],
    ), // 3.125
    CharacterTrait::new_looks("Hideous", [], [], []), // 6.25
    CharacterTrait::new_looks("Unsightly", [], [], []), // 12.5
    CharacterTrait::new_looks("Appealing", [], [], []), // 25
    CharacterTrait::new_looks("Pretty", [], [], []),  // 25
    CharacterTrait::new_looks("Attractive", [], [], []), // 12.5
    CharacterTrait::new_looks("Beautiful", [], [], []), // 6.25
    CharacterTrait::new_looks("Gorgeous", [], [], []), // 3.125
];
