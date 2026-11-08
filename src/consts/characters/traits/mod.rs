use aequa::Object;
use athena::float::HpFloat;

use crate::{
    consts::characters::traits::impact::TraitStatImpact, needs_home::normalise_bvp_name,
    utils::op::Op,
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

/// Maximum amount of specific skills a character trait can impact
/// Limit is chosen arbitrarily but should be kept low in any case.
pub const MAX_CHARACTER_TRAIT_SKILL_IMPACTS: usize = 4;

pub const MAX_CHARACTER_FLAGS: usize = 8;

pub const MAX_CONFLICTING_TRAIT_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    active_conflicting_trait_count: usize,
    conflicting_traits: [&'static str; MAX_CONFLICTING_TRAIT_COUNT],
}

impl PartialEq for CharacterTrait {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind
    }
}

impl Eq for CharacterTrait {}

impl CharacterTrait {
    pub fn apply_to_stats(&self, stats: &mut Object) {
        for impact in self.affected_base_value_pairs().iter() {
            let bvp_name = impact.base_value_name;
            let bvp_value: HpFloat = impact.base_value_value.into();
            let normalised_bvp_names = normalise_bvp_name(bvp_name);
            for normalised_bvp_name in normalised_bvp_names {
                if let Some(to_impact) = stats.get_mut(normalised_bvp_name)
                    && let Some(float) = to_impact.as_hp_float_mut()
                {
                    *float = match impact.op {
                        Op::Add => *float + bvp_value,
                        Op::Sub => *float - bvp_value,
                        Op::Mul => *float * bvp_value,
                        Op::Div => *float / bvp_value,
                        // NOTE: `Set` & `Lim` don't really work if done like this;
                        // They would need to be handled across *all* traits modifying the
                        // same base_value.
                        // TODO: Consider if that trade-off is worth it. I don't
                        // expect Traits to use them much anyways if at all.
                        // Should the trait be the last trait of a character impacting a specific base_value they
                        // almost work as intended (Set should use the largest set
                        // available across all applicable impacts of all traits, same for limit; but hey)
                        Op::Set => bvp_value,
                        Op::Lim => {
                            if *float > bvp_value {
                                bvp_value
                            } else {
                                *float
                            }
                        }
                    };
                }
            }
        }
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
            active_pairs_count: 0,
            affected_base_value_pairs: [TraitStatImpact::const_default();
                MAX_CHARACTER_TRAIT_SKILL_IMPACTS],
            active_flags_count: 0,
            flags: [""; MAX_CHARACTER_FLAGS],
            active_conflicting_trait_count: 0,
            conflicting_traits: [""; MAX_CONFLICTING_TRAIT_COUNT],
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
            O <= MAX_CONFLICTING_TRAIT_COUNT,
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
