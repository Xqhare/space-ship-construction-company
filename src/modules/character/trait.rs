use aequa::{Array, Object, XffSerde, XffValue, xff};
use athena::float::HpFloat;
use tyche::RngApi;

use crate::{
    consts::characters::traits::{
        CharacterTrait, CharacterTraitKind,
        registries::{
            TRAIT_NORMAL_REGISTRY, health::TRAIT_HEALTH_REGISTRY,
            mental_health::TRAIT_MENTAL_HEALTH_REGISTRY, social::TRAIT_SOCIAL_REGISTRY,
            work_ethics::TRAIT_WORK_ETHIC_REGISTRY,
        },
    },
    error::{SSCCResult, new_xff_serde_error},
    needs_home::accum_char_traits,
    utils::Utils,
};

/// Stores all [`CharacterTrait`]s assigned to a [`Character`](super::Character).
///
/// # Trait Slot Categories
/// - **Mandatory Single-Slot (`intellect`, `looks`)**: Always populated. [`CharacterTraitStore::add_trait`]
///   always returns `false` for [`CharacterTraitKind::Intellect`] and [`CharacterTraitKind::Looks`],
///   and [`CharacterTraitStore::remove_trait`] is a no-op for them; they can only be replaced via
///   [`CharacterTraitStore::force_trait`].
/// - **Optional Single-Slot (`health`, `mental_health`, `social`, `work_ethic`)**: Hold at most one
///   trait of their respective [`CharacterTraitKind`]. [`CharacterTraitStore::add_trait`] only
///   inserts if the slot is `None`, whereas [`CharacterTraitStore::force_trait`] overwrites the slot.
/// - **Multi-Slot (`normal_traits`)**: Holds any number of unique, non-conflicting
///   [`CharacterTraitKind::Normal`] traits. [`CharacterTraitStore::add_trait`] rejects duplicates
///   and traits listed in an existing normal trait's [`conflicting_traits`](CharacterTrait::conflicting_traits);
///   [`CharacterTraitStore::force_trait`] removes any duplicate or conflicting normal traits before pushing.
///
/// # Expected Usage & Unenforced Invariants
/// - Marked `#[non_exhaustive]` to encourage construction via [`CharacterTraitStore::new_random`]
///   (or future constructors). When constructed or mutated directly within this crate, each single-slot
///   field is assumed to hold a trait of its matching [`CharacterTraitKind`], and `normal_traits` is
///   assumed to contain no duplicates or symmetric conflicts.
/// - **Do not call `add_trait`, `force_trait`, or `remove_trait` directly on `CharacterTraitStore`
///   for an active [`Character`](super::Character)**: `CharacterTraitStore` only updates trait
///   references and does **not** sync trait flags to `DataBox.flags` or recalculate affinities on
///   `Character.skills`. Always use [`Character::add_trait`](super::Character::add_trait),
///   [`Character::force_trait`](super::Character::force_trait), or
///   [`Character::remove_trait`](super::Character::remove_trait) instead.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterTraitStore {
    pub intellect: &'static CharacterTrait,
    pub looks: &'static CharacterTrait,
    pub health: Option<&'static CharacterTrait>,
    pub mental_health: Option<&'static CharacterTrait>,
    pub social: Option<&'static CharacterTrait>,
    pub work_ethic: Option<&'static CharacterTrait>,
    pub normal_traits: Vec<&'static CharacterTrait>,
}

impl CharacterTraitStore {
    /// Generates a random [`CharacterTraitStore`] given pre-rolled `intellect` and `looks` traits.
    ///
    /// Each optional single-slot category (`health`, `mental_health`, `social`, `work_ethic`) has a
    /// `50%` chance of being populated with a random trait from its registry, followed by `1..=4`
    /// random attempts to add traits from [`TRAIT_NORMAL_REGISTRY`].
    pub fn new_random(
        utils: &mut Utils,
        intellect: &'static CharacterTrait,
        looks: &'static CharacterTrait,
    ) -> SSCCResult<CharacterTraitStore> {
        let mut store = CharacterTraitStore {
            intellect,
            looks,
            health: None,
            mental_health: None,
            social: None,
            work_ethic: None,
            normal_traits: Vec::new(),
        };
        if utils
            .rng
            .random_bool()
            .map_err(|err| err.to_nemesis_error())?
        {
            let _ = store.add_trait(
                TRAIT_HEALTH_REGISTRY[utils
                    .rng
                    .random_index(TRAIT_HEALTH_REGISTRY.len())
                    .map_err(|err| err.to_nemesis_error())?],
            );
        }
        if utils
            .rng
            .random_bool()
            .map_err(|err| err.to_nemesis_error())?
        {
            let _ = store.add_trait(
                TRAIT_MENTAL_HEALTH_REGISTRY[utils
                    .rng
                    .random_index(TRAIT_MENTAL_HEALTH_REGISTRY.len())
                    .map_err(|err| err.to_nemesis_error())?],
            );
        }
        if utils
            .rng
            .random_bool()
            .map_err(|err| err.to_nemesis_error())?
        {
            let _ = store.add_trait(
                TRAIT_SOCIAL_REGISTRY[utils
                    .rng
                    .random_index(TRAIT_SOCIAL_REGISTRY.len())
                    .map_err(|err| err.to_nemesis_error())?],
            );
        }
        if utils
            .rng
            .random_bool()
            .map_err(|err| err.to_nemesis_error())?
        {
            let _ = store.add_trait(
                TRAIT_WORK_ETHIC_REGISTRY[utils
                    .rng
                    .random_index(TRAIT_WORK_ETHIC_REGISTRY.len())
                    .map_err(|err| err.to_nemesis_error())?],
            );
        }
        let normal_trait_amount = utils
            .rng
            .random_from_range_inclusive(1, 4)
            .map_err(|err| err.to_nemesis_error())?;
        for _ in 0..normal_trait_amount {
            let _ = store.add_trait(
                TRAIT_NORMAL_REGISTRY[utils
                    .rng
                    .random_index(TRAIT_NORMAL_REGISTRY.len())
                    .map_err(|err| err.to_nemesis_error())?],
            );
        }
        Ok(store)
    }
    /// Removes `character_trait` from the store if present.
    ///
    /// Mandatory traits (`Intellect` and `Looks`) cannot be removed and are ignored.
    pub fn remove_trait(&mut self, character_trait: &'static CharacterTrait) {
        if self.has_trait(character_trait.name) {
            match character_trait.kind {
                CharacterTraitKind::Intellect | CharacterTraitKind::Looks => {}
                CharacterTraitKind::Health => self.health = None,
                CharacterTraitKind::MentalHealth => self.mental_health = None,
                CharacterTraitKind::Social => self.social = None,
                CharacterTraitKind::WorkEthics => self.work_ethic = None,
                CharacterTraitKind::Normal => {
                    self.normal_traits
                        .retain(|existing| *existing != character_trait);
                }
            }
        }
    }
    /// Computes the `(flat_accum, perc_accum)` stat impact accumulators across all traits in the store.
    pub fn make_all_traits_accum(&self) -> Vec<(&'static str, (HpFloat, HpFloat))> {
        accum_char_traits(&self.get_all_traits())
    }
    /// Returns a `Vec` containing all active traits in the store (all `normal_traits` plus
    /// `intellect`, `looks`, and any populated optional single-slot traits).
    pub fn get_all_traits(&self) -> Vec<&'static CharacterTrait> {
        // Reserve capacity for `normal_traits` plus up to 6 single-slot traits
        let mut out = Vec::with_capacity(self.normal_traits.len() + 6);
        out.extend(self.normal_traits.clone());
        out.push(self.intellect);
        out.push(self.looks);
        if let Some(r#trait) = self.health {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.mental_health {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.social {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.work_ethic {
            out.push(r#trait);
        }
        out
    }
    pub fn get_trait(&self, character_trait_name: &'static str) -> Option<&'static CharacterTrait> {
        if character_trait_name == self.intellect.name {
            Some(self.intellect)
        } else if character_trait_name == self.looks.name {
            Some(self.looks)
        } else if let Some(r#trait) = self.health
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.mental_health
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.social
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.work_ethic
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else {
            for r#trait in &self.normal_traits {
                if r#trait.name == character_trait_name {
                    return Some(r#trait);
                }
            }
            None
        }
    }
    pub fn has_trait(&self, character_trait_name: &'static str) -> bool {
        self.get_trait(character_trait_name).is_some()
    }
    /// Returns `true` if the trait was added.
    pub fn add_trait(&mut self, character_trait: &'static CharacterTrait) -> bool {
        match character_trait.kind {
            CharacterTraitKind::Looks | CharacterTraitKind::Intellect => false,
            CharacterTraitKind::Normal => {
                let has_duplicate_or_conflict = self.normal_traits.iter().any(|existing| {
                    *existing == character_trait
                        || character_trait
                            .conflicting_traits()
                            .contains(&existing.name)
                });
                if !has_duplicate_or_conflict {
                    self.normal_traits.push(character_trait);
                    true
                } else {
                    false
                }
            }
            _ => {
                let store = match character_trait.kind {
                    CharacterTraitKind::Looks
                    | CharacterTraitKind::Intellect
                    | CharacterTraitKind::Normal => {
                        unreachable!(
                            "CharacterTraitStore::add_trait, Match arm should be unreachable!"
                        )
                    }
                    CharacterTraitKind::Health => &mut self.health,
                    CharacterTraitKind::MentalHealth => &mut self.mental_health,
                    CharacterTraitKind::Social => &mut self.social,
                    CharacterTraitKind::WorkEthics => &mut self.work_ethic,
                };
                if store.is_none() {
                    *store = Some(character_trait);
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn force_trait(&mut self, character_trait: &'static CharacterTrait) {
        match character_trait.kind {
            CharacterTraitKind::Intellect => self.intellect = character_trait,
            CharacterTraitKind::Looks => self.looks = character_trait,
            CharacterTraitKind::Health => self.health = Some(character_trait),
            CharacterTraitKind::MentalHealth => self.mental_health = Some(character_trait),
            CharacterTraitKind::Social => self.social = Some(character_trait),
            CharacterTraitKind::WorkEthics => self.work_ethic = Some(character_trait),
            CharacterTraitKind::Normal => {
                self.normal_traits.retain(|existing| {
                    *existing != character_trait
                        && !character_trait
                            .conflicting_traits()
                            .contains(&existing.name)
                });
                self.normal_traits.push(character_trait);
            }
        }
    }
}
const INTELLECT_FIELD: &str = "int";
const LOOKS_FIELD: &str = "looks";
const HEALTH_FIELD: &str = "health";
const MENTAL_HEALTH_FIELD: &str = "mental_health";
const SOCIAL_FIELD: &str = "social";
const WORK_ETHIC_FIELD: &str = "work";
const NORMAL_TRAITS_FIELD: &str = "normals";
impl XffSerde for CharacterTraitStore {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert(INTELLECT_FIELD, self.intellect.name);
        obj.insert(LOOKS_FIELD, self.looks.name);
        if let Some(health) = self.health {
            obj.insert(HEALTH_FIELD, health.name);
        }
        if let Some(mental_health) = self.mental_health {
            obj.insert(MENTAL_HEALTH_FIELD, mental_health.name);
        }
        if let Some(social) = self.social {
            obj.insert(SOCIAL_FIELD, social.name);
        }
        if let Some(work) = self.work_ethic {
            obj.insert(WORK_ETHIC_FIELD, work.name);
        }
        let normals = {
            let mut ary = Array::new();
            for normal in &self.normal_traits {
                ary.push(normal.name);
            }
            ary
        };
        obj.insert(NORMAL_TRAITS_FIELD, normals);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(int) = obj.get(INTELLECT_FIELD)
                && let Some(int) = int.as_string()
                && let Some(intellect) = CharacterTrait::from_trait_name(&int)
                && let Some(looks) = obj.get(LOOKS_FIELD)
                && let Some(looks) = looks.as_string()
                && let Some(looks) = CharacterTrait::from_trait_name(&looks)
                && let Some(normals) = obj.get(NORMAL_TRAITS_FIELD)
                && let Some(normals) = normals.as_array()
            {
                let normal_traits = {
                    let mut tmp = Vec::with_capacity(normals.len());
                    for normal in normals {
                        tmp.push(
                            CharacterTrait::from_trait_name(&normal.to_string()).ok_or(
                                new_xff_serde_error(&format!(
                                    "Cannot parse '{}' as a valid trait.",
                                    normal.to_string(),
                                ))
                                .add_ctx(format!("XffValue: {xff:?}")),
                            )?,
                        )
                    }
                    tmp
                };
                let health = {
                    if let Some(health) = obj.get(HEALTH_FIELD) {
                        Some(
                            CharacterTrait::from_trait_name(&health.to_string()).ok_or(
                                new_xff_serde_error(&format!(
                                    "Cannot parse '{}' as a valid trait.",
                                    health.to_string(),
                                ))
                                .add_ctx(format!("XffValue: {xff:?}")),
                            )?,
                        )
                    } else {
                        None
                    }
                };
                let mental_health = {
                    if let Some(mental_health) = obj.get(MENTAL_HEALTH_FIELD) {
                        Some(
                            CharacterTrait::from_trait_name(&mental_health.to_string()).ok_or(
                                new_xff_serde_error(&format!(
                                    "Cannot parse '{}' as a valid trait.",
                                    mental_health.to_string(),
                                ))
                                .add_ctx(format!("XffValue: {xff:?}")),
                            )?,
                        )
                    } else {
                        None
                    }
                };
                let social = {
                    if let Some(social) = obj.get(SOCIAL_FIELD) {
                        Some(
                            CharacterTrait::from_trait_name(&social.to_string()).ok_or(
                                new_xff_serde_error(&format!(
                                    "Cannot parse '{}' as a valid trait.",
                                    social.to_string(),
                                ))
                                .add_ctx(format!("XffValue: {xff:?}")),
                            )?,
                        )
                    } else {
                        None
                    }
                };
                let work_ethic = {
                    if let Some(work) = obj.get(WORK_ETHIC_FIELD) {
                        Some(
                            CharacterTrait::from_trait_name(&work.to_string()).ok_or(
                                new_xff_serde_error(&format!(
                                    "Cannot parse '{}' as a valid trait.",
                                    work.to_string(),
                                ))
                                .add_ctx(format!("XffValue: {xff:?}")),
                            )?,
                        )
                    } else {
                        None
                    }
                };
                Ok(CharacterTraitStore {
                    intellect,
                    looks,
                    health,
                    mental_health,
                    social,
                    work_ethic,
                    normal_traits,
                })
            } else {
                Err(new_xff_serde_error(
                    "Cannot parse CharacterTraitStore; Required fields missing!",
                )
                .add_ctx(format!("Value: '{xff:?}'")))
            }
        } else {
            Err(
                new_xff_serde_error("Cannot parse CharacterTraitStore; XffValue not an object!")
                    .add_ctx(format!("{xff:?}")),
            )
        }
    }
}
