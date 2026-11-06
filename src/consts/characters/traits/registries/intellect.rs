use crate::consts::{
    base_values::science::SCI_BONUS_ALL_FIELDS,
    characters::{
        base_values::SOCIABILITY,
        flags::{ALWAYS_PROVIDES_ALL, ALWAYS_PROVIDES_RESEARCH},
        traits::{CharacterTrait, TraitStatImpact},
    },
};
pub const TRAIT_INTELLECT_GENIUS: CharacterTrait = CharacterTrait::new(
    "Genius",
    [
        TraitStatImpact::new_flat((SCI_BONUS_ALL_FIELDS.0, Some(7.5))),
        TraitStatImpact::new((SOCIABILITY.0, Some(-10.0))),
    ],
    [ALWAYS_PROVIDES_ALL],
);
pub const TRAIT_INTELLECT_SHREWD: CharacterTrait = CharacterTrait::new(
    "Shrewd",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(2.5),
    ))],
    [ALWAYS_PROVIDES_ALL],
);
pub const TRAIT_INTELLECT_ASTUTE: CharacterTrait = CharacterTrait::new(
    "Astute",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(5.0),
    ))],
    [ALWAYS_PROVIDES_RESEARCH],
);
pub const TRAIT_INTELLECT_BRIGHT: CharacterTrait = CharacterTrait::new(
    "Bright",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(0.5),
    ))],
    [ALWAYS_PROVIDES_RESEARCH],
);
pub const TRAIT_INTELLECT_DENSE: CharacterTrait = CharacterTrait::new(
    "Dense",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(-2.5),
    ))],
    [],
);
pub const TRAIT_INTELLECT_DIM: CharacterTrait = CharacterTrait::new(
    "Dim",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(-0.5),
    ))],
    [],
);
pub const TRAIT_INTELLECT_SMART: CharacterTrait = CharacterTrait::new(
    "Smart",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(1.5),
    ))],
    [ALWAYS_PROVIDES_RESEARCH],
);
pub const TRAIT_INTELLECT_SIMPLETON: CharacterTrait = CharacterTrait::new(
    "Simpleton",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(-7.5),
    ))],
    [],
);
pub const TRAIT_INTELLECT_DULL: CharacterTrait = CharacterTrait::new(
    "Dull",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(-1.5),
    ))],
    [],
);
pub const TRAIT_INTELLECT_DIMWIT: CharacterTrait = CharacterTrait::new(
    "Dimwit",
    [TraitStatImpact::new_flat((
        SCI_BONUS_ALL_FIELDS.0,
        Some(-5.0),
    ))],
    [],
);

pub static TRAIT_INTELLECT_IQ_REGISTRY: [CharacterTrait; 10] = [
    // Ck3 Traits
    TRAIT_INTELLECT_GENIUS,
    TRAIT_INTELLECT_SHREWD,
    // Ck 2 Rolls Community Mod Traits
    TRAIT_INTELLECT_ASTUTE,
    TRAIT_INTELLECT_BRIGHT,
    TRAIT_INTELLECT_DENSE,
    TRAIT_INTELLECT_DIM,
    TRAIT_INTELLECT_SMART,
    // My additions
    TRAIT_INTELLECT_SIMPLETON,
    TRAIT_INTELLECT_DULL,
    TRAIT_INTELLECT_DIMWIT,
];
