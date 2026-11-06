use crate::consts::characters::traits::impact::TraitStatImpact;

pub mod impact;
pub mod registries;

/// Maximum amount of specific skills a character trait can impact
/// Limit is chosen arbitrarily but should be kept low in any case.
pub const MAX_CHARACTER_TRAIT_SKILL_IMPACTS: usize = 4;

pub const MAX_CHARACTER_FLAGS: usize = 8;

pub const MAX_CONFLICTING_TRAIT_COUNT: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum CharacterTraitKind {
    Intellect,
    Looks,
    Health,
    MentalHealth,
    Social,
    WorkEthics,
    Normal,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct CharacterTrait {
    pub name: &'static str,
    pub kind: CharacterTraitKind,
    active_pairs_count: usize,
    affected_base_value_pairs: [TraitStatImpact; MAX_CHARACTER_TRAIT_SKILL_IMPACTS],
    active_flags_count: usize,
    flags: [&'static str; MAX_CHARACTER_FLAGS],
    active_conflicitng_trait_count: usize,
    conflicting_traits: [&'static str; MAX_CONFLICTING_TRAIT_COUNT],
}

impl CharacterTrait {
    pub const fn new_normal<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::Normal,
        )
    }
    pub const fn new_health<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::Health,
        )
    }
    pub const fn new_mental_health<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::MentalHealth,
        )
    }
    pub const fn new_intellect<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::Intellect,
        )
    }
    pub const fn new_social<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::Social,
        )
    }
    pub const fn new_work_ethic<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::WorkEthics,
        )
    }
    pub const fn new_looks<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
    ) -> CharacterTrait {
        Self::new(
            name,
            affected_base_value_pairs,
            flags,
            conflicting_trait_names,
            CharacterTraitKind::Looks,
        )
    }
    const fn new<const N: usize, const M: usize, const O: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
        conflicting_trait_names: [&'static str; O],
        trait_kind: CharacterTraitKind,
    ) -> CharacterTrait {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            N <= MAX_CHARACTER_TRAIT_SKILL_IMPACTS,
            "Trait has too many skill impacts"
        );
        assert!(M <= MAX_CHARACTER_FLAGS, "Trait has too many flags");
        assert!(
            O <= MAX_CONFLICTING_TRAIT_COUNT,
            "Trait has too many conflicting traits"
        );
        let mut impacted_pairs =
            [TraitStatImpact::const_default(); MAX_CHARACTER_TRAIT_SKILL_IMPACTS];
        let mut i = 0;
        while i < N {
            impacted_pairs[i] = affected_base_value_pairs[i];
            i += 1;
        }
        let mut impacted_flags = [""; MAX_CHARACTER_FLAGS];
        i = 0;
        while i < M {
            impacted_flags[i] = flags[i];
            i += 1;
        }
        let mut conflicts = [""; MAX_CONFLICTING_TRAIT_COUNT];
        i = 0;
        while i < O {
            conflicts[i] = conflicting_trait_names[i];
            i += 1;
        }
        CharacterTrait {
            name,
            active_pairs_count: N,
            affected_base_value_pairs: impacted_pairs,
            active_flags_count: M,
            flags: impacted_flags,
            active_conflicitng_trait_count: O,
            conflicting_traits: conflicts,
            kind: trait_kind,
        }
    }
    pub fn affected_base_value_pairs(&self) -> &[TraitStatImpact] {
        &self.affected_base_value_pairs[..self.active_pairs_count]
    }
    pub fn flags(&self) -> &[&str] {
        &self.flags[..self.active_flags_count]
    }
    pub fn conflicting_traits(&self) -> &[&'static str] {
        &self.conflicting_traits[..self.active_conflicitng_trait_count]
    }
}
