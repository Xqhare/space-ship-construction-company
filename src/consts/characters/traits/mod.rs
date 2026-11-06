use crate::{
    consts::{ConstBVPair, EMPTY_BV_PAIR},
    modules::modifier::{op::ModifierOp, r#type::ModifierType},
};

pub mod registries;

/// Maximum amount of specific skills a character trait can impact
/// Limit is chosen arbitrarily but should be kept low in any case.
pub const MAX_CHARACTER_TRAIT_SKILL_IMPACTS: usize = 4;

pub const MAX_CHARACTER_FLAGS: usize = 8;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct CharacterTrait {
    pub name: &'static str,
    active_pairs_count: usize,
    affected_base_value_pairs: [TraitStatImpact; MAX_CHARACTER_TRAIT_SKILL_IMPACTS],
    active_flags_count: usize,
    flags: [&'static str; MAX_CHARACTER_FLAGS],
}

#[derive(Debug, Clone, Copy)]
pub struct TraitStatImpact {
    pub base_value_pair: ConstBVPair,
    pub kind: ModifierType,
    pub op: ModifierOp,
}

impl TraitStatImpact {
    const fn const_default() -> TraitStatImpact {
        TraitStatImpact {
            base_value_pair: EMPTY_BV_PAIR,
            kind: ModifierType::Percentage,
            op: ModifierOp::Add,
        }
    }
    /// Creates a new stat impact.
    ///
    /// The kind will be `ModifierType::Percentage`.
    /// The op will be `ModifierOp::Add`.
    ///
    /// For more info see [`StatBox::get_all_stats()`]
    pub const fn new(base_value_pair: ConstBVPair) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::const_default();
        tsi.base_value_pair = base_value_pair;
        tsi
    }
    /// Create a new stat impact of kind `ModifierType::Flat`.
    ///
    /// The op will be `ModififerOp::Add`.
    pub const fn new_flat(base_value_pair: ConstBVPair) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::new(base_value_pair);
        tsi.kind = ModifierType::Flat;
        tsi
    }
    /// Create a fully custom stat impact
    pub const fn custom(
        base_value_pair: ConstBVPair,
        kind: ModifierType,
        op: ModifierOp,
    ) -> TraitStatImpact {
        TraitStatImpact {
            base_value_pair,
            kind,
            op,
        }
    }
}

impl CharacterTrait {
    pub const fn new<const N: usize, const M: usize>(
        name: &'static str,
        affected_base_value_pairs: [TraitStatImpact; N],
        flags: [&'static str; M],
    ) -> CharacterTrait {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            N <= MAX_CHARACTER_TRAIT_SKILL_IMPACTS,
            "Trait has too many skill impacts"
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
        CharacterTrait {
            name,
            active_pairs_count: N,
            affected_base_value_pairs: impacted_pairs,
            active_flags_count: M,
            flags: impacted_flags,
        }
    }
    pub fn affected_base_value_pairs(&self) -> &[TraitStatImpact] {
        &self.affected_base_value_pairs[..self.active_pairs_count]
    }
    pub fn flags(&self) -> &[&str] {
        &self.flags[..self.active_flags_count]
    }
}
