use crate::consts::characters::traits::CharacterTrait;

pub static TRAIT_LOOKS_REGISTRY: [CharacterTrait; 8] = [
    CharacterTrait::new("Aberrant", [], []),   // 3.125
    CharacterTrait::new("Hideous", [], []),    // 6.25
    CharacterTrait::new("Unsightly", [], []),  // 12.5
    CharacterTrait::new("Appealing", [], []),  // 25
    CharacterTrait::new("Pretty", [], []),     // 25
    CharacterTrait::new("Attractive", [], []), // 12.5
    CharacterTrait::new("Beautiful", [], []),  // 6.25
    CharacterTrait::new("Gorgeous", [], []),   // 3.125
];
