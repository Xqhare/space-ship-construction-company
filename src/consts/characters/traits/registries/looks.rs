use crate::consts::characters::{
    base_values::SOCIABILITY,
    traits::{CharacterTrait, impact::TraitStatImpact},
};

define_trait_registry! {
    pub static TRAIT_LOOKS_REGISTRY = [
        TRAIT_LOOKS_ABERRANT   => CharacterTrait::looks("Aberrant")
            .with_impacts([TraitStatImpact::new(SOCIABILITY, -0.1)]), // 3.125
        TRAIT_LOOKS_HIDEOUS    => CharacterTrait::looks("Hideous"),   // 6.25
        TRAIT_LOOKS_UNSIGHTLY  => CharacterTrait::looks("Unsightly"), // 12.5
        TRAIT_LOOKS_APPEALING  => CharacterTrait::looks("Appealing"), // 25
        TRAIT_LOOKS_PRETTY     => CharacterTrait::looks("Pretty"),    // 25
        TRAIT_LOOKS_ATTRACTIVE => CharacterTrait::looks("Attractive"),// 12.5
        TRAIT_LOOKS_BEAUTIFUL  => CharacterTrait::looks("Beautiful"), // 6.25
        TRAIT_LOOKS_GORGEOUS   => CharacterTrait::looks("Gorgeous"),  // 3.125
    ];
}
