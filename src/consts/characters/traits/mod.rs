use crate::consts::characters::traits::{
    impact::TraitStatImpact, registries::get_trait_from_registry_by_name,
};

macro_rules! define_trait_registry {
    (
        pub static $registry_name:ident = [
            $(
                $trait_const:ident => $trait_expr:expr
            ),* $(,)?
        ];
    ) => {
        $(
            pub static $trait_const: $crate::consts::characters::traits::CharacterTrait = $trait_expr;
        )*

        pub static $registry_name: &[&$crate::consts::characters::traits::CharacterTrait] = &[
            $( &$trait_const ),*
        ];
    };
}

pub mod impact;
pub mod registries;

/// Maximum number of [`TraitStatImpact`]s a [`CharacterTrait`] can define (enforced at compile time).
pub const MAX_CHARACTER_TRAIT_SKILL_IMPACTS: usize = 4;

/// Maximum number of flags a [`CharacterTrait`] can define (enforced at compile time).
pub const MAX_CHARACTER_FLAGS: usize = 8;

/// Maximum number of conflicting trait names a [`CharacterTrait`] can define (enforced at compile time).
pub const MAX_CHARACTER_CONFLICTING_TRAIT_COUNT: usize = 4;

/// Maximum number of [`TraitAffinityImpact`]s a [`CharacterTrait`] can define (enforced at compile time).
pub const MAX_CHARACTER_TRAIT_AFFINITY_TARGETS: usize = 4;

/// Categorizes which slot(s) in [`CharacterTraitStore`](crate::modules::character::trait::CharacterTraitStore)
/// a [`CharacterTrait`] occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTraitKind {
    /// Mandatory single-slot trait rolled at character creation.
    Intellect,
    /// Mandatory single-slot trait rolled at character creation.
    Looks,
    /// Optional single-slot trait.
    Health,
    /// Optional single-slot trait.
    MentalHealth,
    /// Optional single-slot trait.
    Social,
    /// Optional single-slot trait.
    WorkEthics,
    /// Multi-slot general trait (subject to [`CharacterTrait::conflicting_traits`] checks).
    Normal,
}

/// Specifies which skill(s) a [`TraitAffinityImpact`] modifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffinityTarget {
    /// Internal default placeholder for unused array slots.
    ///
    /// Must never be used in an active [`TraitAffinityImpact`]; encountering `AffinityTarget::None`
    /// during [`CharacterSkill`](crate::modules::character::skill::CharacterSkill) affinity calculation
    /// triggers a `debug_assert!` failure in debug builds (and is ignored in release builds).
    None,
    /// Applies to a specific skill by name (`1.0 * delta`), e.g. `SKILL_SOCIAL_LYING.name`.
    Skill(&'static str),
    /// Applies to any skill whose `master` (`1.0 * delta`) or `secondaries` (`0.25 * delta` per match)
    /// impact this base value. Supports group keys like `SCI_BONUS_ALL_FIELDS.0` via
    /// [`normalise_bvp_name`](crate::consts::normalise_bvp_name).
    BaseValue(&'static str),
    /// Applies to every skill (`1.0 * delta`), e.g. Genius or Dimwit.
    AllSkills,
}

/// Defines a trait's additive contribution (`delta`) to skill affinity for a given [`AffinityTarget`].
///
/// # Expected Range (`0.0..=1.0`, unenforced / soft-capped)
/// Across all traits on a character (plus the `0.0125` base affinity), the summed affinity for a
/// skill is expected to fall within `0.0..=1.0`. See [`CharacterSkill`](crate::modules::character::skill::CharacterSkill)
/// for how values outside `0.0..=1.0` behave.
#[derive(Debug, Clone, Copy)]
pub struct TraitAffinityImpact {
    pub target: AffinityTarget,
    pub delta: f32,
}

impl TraitAffinityImpact {
    pub const fn const_default() -> TraitAffinityImpact {
        TraitAffinityImpact {
            target: AffinityTarget::None,
            delta: 0.0,
        }
    }
    pub const fn new(target: AffinityTarget, delta: f32) -> TraitAffinityImpact {
        TraitAffinityImpact { target, delta }
    }
}

/// Compile-time definition of a character trait, including its slot kind, skill affinity deltas,
/// stat impacts, flags, and conflicting trait names.
///
/// # Expected Usage & Registry Invariants
/// - Every `CharacterTrait` must be registered in [`TRAIT_REGISTRY_OF_REGISTRIES`](registries::TRAIT_REGISTRY_OF_REGISTRIES)
///   with a globally unique `name`.
/// - All entries in [`CharacterTrait::conflicting_traits`] must exist in a registry and be symmetric
///   (if trait `A` lists `B` as conflicting, `B` must also list `A`; verified by unit tests in
///   [`registries`]).
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct CharacterTrait {
    pub name: &'static str,
    pub kind: CharacterTraitKind,
    active_affinity_target_count: usize,
    affinity_targets: [TraitAffinityImpact; MAX_CHARACTER_TRAIT_AFFINITY_TARGETS],
    active_pairs_count: usize,
    affected_base_value_pairs: [TraitStatImpact; MAX_CHARACTER_TRAIT_SKILL_IMPACTS],
    active_flags_count: usize,
    flags: [&'static str; MAX_CHARACTER_FLAGS],
    active_conflicting_trait_count: usize,
    conflicting_traits: [&'static str; MAX_CHARACTER_CONFLICTING_TRAIT_COUNT],
}

impl PartialEq for CharacterTrait {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind
    }
}

impl Eq for CharacterTrait {}

impl CharacterTrait {
    pub fn from_trait_name(trait_name: &str) -> Option<&'static CharacterTrait> {
        get_trait_from_registry_by_name(trait_name)
    }
    pub const fn normal(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::Normal)
    }
    pub const fn health(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::Health)
    }
    pub const fn mental_health(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::MentalHealth)
    }
    pub const fn intellect(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::Intellect)
    }
    pub const fn social(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::Social)
    }
    pub const fn work_ethic(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::WorkEthics)
    }
    pub const fn looks(name: &'static str) -> CharacterTrait {
        Self::new(name, CharacterTraitKind::Looks)
    }
    const fn new(name: &'static str, kind: CharacterTraitKind) -> CharacterTrait {
        CharacterTrait {
            name,
            kind,
            active_affinity_target_count: 0,
            affinity_targets: [TraitAffinityImpact::const_default();
                MAX_CHARACTER_TRAIT_AFFINITY_TARGETS],
            active_pairs_count: 0,
            affected_base_value_pairs: [TraitStatImpact::const_default();
                MAX_CHARACTER_TRAIT_SKILL_IMPACTS],
            active_flags_count: 0,
            flags: [""; MAX_CHARACTER_FLAGS],
            active_conflicting_trait_count: 0,
            conflicting_traits: [""; MAX_CHARACTER_CONFLICTING_TRAIT_COUNT],
        }
    }
    pub const fn with_impacts<const N: usize>(
        mut self,
        affected_base_value_pairs: [TraitStatImpact; N],
    ) -> CharacterTrait {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            N <= MAX_CHARACTER_TRAIT_SKILL_IMPACTS,
            "Trait has too many skill impacts"
        );
        let mut i = 0;
        while i < N {
            self.affected_base_value_pairs[i] = affected_base_value_pairs[i];
            i += 1;
        }
        self.active_pairs_count = N;
        self
    }
    pub const fn with_flags<const M: usize>(mut self, flags: [&'static str; M]) -> CharacterTrait {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(M <= MAX_CHARACTER_FLAGS, "Trait has too many flags");
        let mut i = 0;
        while i < M {
            self.flags[i] = flags[i];
            i += 1;
        }
        self.active_flags_count = M;
        self
    }
    pub const fn with_conflicts<const O: usize>(
        mut self,
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            O <= MAX_CHARACTER_CONFLICTING_TRAIT_COUNT,
            "Trait has too many conflicting traits"
        );
        let mut i = 0;
        while i < O {
            self.conflicting_traits[i] = conflicting_trait_names[i];
            i += 1;
        }
        self.active_conflicting_trait_count = O;
        self
    }
    pub const fn with_affinities<const P: usize>(
        mut self,
        affinities: [TraitAffinityImpact; P],
    ) -> CharacterTrait {
        assert!(
            P <= MAX_CHARACTER_TRAIT_AFFINITY_TARGETS,
            "Trait has too many affinities defined!"
        );
        let mut i = 0;
        while i < P {
            self.affinity_targets[i] = affinities[i];
            i += 1;
        }
        self.active_affinity_target_count = P;
        self
    }
    pub fn affinities(&self) -> &[TraitAffinityImpact] {
        &self.affinity_targets[..self.active_affinity_target_count]
    }
    pub fn affected_base_value_pairs(&self) -> &[TraitStatImpact] {
        &self.affected_base_value_pairs[..self.active_pairs_count]
    }
    pub fn flags(&self) -> &[&str] {
        &self.flags[..self.active_flags_count]
    }
    pub fn conflicting_traits(&self) -> &[&'static str] {
        &self.conflicting_traits[..self.active_conflicting_trait_count]
    }
}
