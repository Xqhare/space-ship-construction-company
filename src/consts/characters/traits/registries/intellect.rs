use crate::consts::{
    base_values::science::SCI_BONUS_ALL_FIELDS,
    characters::{
        base_values::SOCIABILITY,
        flags::{ALWAYS_PROVIDES_ALL, ALWAYS_PROVIDES_RESEARCH},
        traits::{CharacterTrait, impact::TraitStatImpact},
    },
};

define_trait_registry! {
    pub static TRAIT_INTELLECT_IQ_REGISTRY = [
        // Ck3 Traits
        TRAIT_INTELLECT_GENIUS => CharacterTrait::intellect("Genius")
            .with_impacts([
                TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, 7.5),
                TraitStatImpact::new(SOCIABILITY, -10.0),
            ])
            .with_flags([ALWAYS_PROVIDES_ALL]),
        TRAIT_INTELLECT_SHREWD => CharacterTrait::intellect("Shrewd")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, 2.5)])
            .with_flags([ALWAYS_PROVIDES_ALL]),
        // Ck2 Rolls Community Mod Traits
        TRAIT_INTELLECT_ASTUTE => CharacterTrait::intellect("Astute")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, 5.0)])
            .with_flags([ALWAYS_PROVIDES_RESEARCH]),
        TRAIT_INTELLECT_BRIGHT => CharacterTrait::intellect("Bright")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, 0.5)])
            .with_flags([ALWAYS_PROVIDES_RESEARCH]),
        TRAIT_INTELLECT_DENSE => CharacterTrait::intellect("Dense")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, -2.5)]),
        TRAIT_INTELLECT_DIM => CharacterTrait::intellect("Dim")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, -0.5)]),
        TRAIT_INTELLECT_SMART => CharacterTrait::intellect("Smart")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, 1.5)])
            .with_flags([ALWAYS_PROVIDES_RESEARCH]),
        // Custom additions
        TRAIT_INTELLECT_SIMPLETON => CharacterTrait::intellect("Simpleton")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, -7.5)]),
        TRAIT_INTELLECT_DULL => CharacterTrait::intellect("Dull")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, -1.5)]),
        TRAIT_INTELLECT_DIMWIT => CharacterTrait::intellect("Dimwit")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS, -5.0)]),
    ];
}
