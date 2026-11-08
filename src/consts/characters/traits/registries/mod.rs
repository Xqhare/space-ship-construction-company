use crate::consts::{
    base_values::science::SCI_BONUS_ALL_FIELDS,
    characters::{
        flags::{ALWAYS_PROVIDES_RESEARCH, suicide::SUICIDE_NEVER_ON_SITE},
        traits::{
            CharacterTrait,
            impact::TraitStatImpact,
            registries::{
                health::TRAIT_HEALTH_REGISTRY, intellect::TRAIT_INTELLECT_IQ_REGISTRY,
                looks::TRAIT_LOOKS_REGISTRY, mental_health::TRAIT_MENTAL_HEALTH_REGISTRY,
                social::TRAIT_SOCIAL_REGISTRY, work_ethics::TRAIT_WORK_ETHIC_REGISTRY,
            },
        },
    },
};

// TODO: Go through all traits and make their modifier values make sense
pub mod health;
pub mod intellect;
pub mod looks;
pub mod mental_health;
pub mod social;
pub mod work_ethics;

pub static TRAIT_REGISTRY_OF_REGISTRIES: [&[&CharacterTrait]; 7] = [
    TRAIT_INTELLECT_IQ_REGISTRY,
    TRAIT_LOOKS_REGISTRY,
    TRAIT_HEALTH_REGISTRY,
    TRAIT_MENTAL_HEALTH_REGISTRY,
    TRAIT_SOCIAL_REGISTRY,
    TRAIT_WORK_ETHIC_REGISTRY,
    TRAIT_NORMAL_REGISTRY,
];

pub fn get_trait_from_registry_by_name(trait_name: &str) -> Option<&'static CharacterTrait> {
    TRAIT_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .find(|entry| entry.name == trait_name)
}

define_trait_registry! {
    pub static TRAIT_NORMAL_REGISTRY = [
        TRAIT_NORMAL_CURIOUS => CharacterTrait::normal("Curious")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS.0, 1.0)])
            .with_flags([ALWAYS_PROVIDES_RESEARCH])
            .with_conflicts(["Incurious"]),
        TRAIT_NORMAL_INCURIOUS => CharacterTrait::normal("Incurious")
            .with_impacts([TraitStatImpact::new_flat(SCI_BONUS_ALL_FIELDS.0, -1.0)])
            .with_conflicts(["Curious"]),
        TRAIT_NORMAL_NUMBERPHILE => CharacterTrait::normal("Numberphile"),
        TRAIT_NORMAL_STRATEGIST  => CharacterTrait::normal("Strategist"),
        TRAIT_NORMAL_TACTICIAN   => CharacterTrait::normal("Tactician"),
        TRAIT_NORMAL_PENSIVE     => CharacterTrait::normal("Pensive")
            .with_flags([SUICIDE_NEVER_ON_SITE]),
        TRAIT_NORMAL_ERUDITE     => CharacterTrait::normal("Erudite"),
        // Placeholders / need to earn their place
        TRAIT_NORMAL_BROODING    => CharacterTrait::normal("Brooding"),
        TRAIT_NORMAL_GLUTTONOUS  => CharacterTrait::normal("Gluttonous"),
        TRAIT_NORMAL_CYNICAL     => CharacterTrait::normal("Cynical"),
        TRAIT_NORMAL_DECADENT    => CharacterTrait::normal("Decadent"),
        TRAIT_NORMAL_HAUGHTY     => CharacterTrait::normal("Haughty"),
        TRAIT_NORMAL_IDOLIZER    => CharacterTrait::normal("Idolizer"),
        TRAIT_NORMAL_PLAYFUL     => CharacterTrait::normal("Playful"),
        TRAIT_NORMAL_UNCOUTH     => CharacterTrait::normal("Uncouth"),
        TRAIT_NORMAL_WILLFUL     => CharacterTrait::normal("Willful"),
        // Conflicting
        TRAIT_NORMAL_ARBITRARY   => CharacterTrait::normal("Arbitrary")
            .with_conflicts(["Just"]),
        TRAIT_NORMAL_JUST        => CharacterTrait::normal("Just")
            .with_conflicts(["Arbitrary"]),
        TRAIT_NORMAL_BRAVE       => CharacterTrait::normal("Brave")
            .with_conflicts(["Craven"]),
        TRAIT_NORMAL_CRAVEN      => CharacterTrait::normal("Craven")
            .with_conflicts(["Brave"]),
        TRAIT_NORMAL_TEMPERATE   => CharacterTrait::normal("Temperate")
            .with_conflicts(["Proud"]),
        TRAIT_NORMAL_PROUD       => CharacterTrait::normal("Proud")
            .with_conflicts(["Temperate"]),
        TRAIT_NORMAL_ROWDY       => CharacterTrait::normal("Rowdy")
            .with_conflicts(["Calm"]),
        TRAIT_NORMAL_CALM        => CharacterTrait::normal("Calm")
            .with_conflicts(["Rowdy"]),
    ];
}

#[test]
fn all_conflicting_links_exist_and_are_symmetric() {
    for reg in TRAIT_REGISTRY_OF_REGISTRIES {
        for entry in reg {
            for &con in entry.conflicting_traits() {
                if let Some(target) = get_trait_from_registry_by_name(con) {
                    assert!(
                        target.conflicting_traits().contains(&entry.name),
                        "Trait '{}' conflicts with '{}', but '{}' does not list '{}' in its conflicting_traits",
                        entry.name,
                        con,
                        target.name,
                        entry.name
                    );
                } else {
                    panic!(
                        "Trait '{}' conflicts with '{}', which does not exist in any registry",
                        entry.name, con
                    );
                };
            }
        }
    }
}
