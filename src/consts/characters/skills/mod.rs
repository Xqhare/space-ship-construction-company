use athena::float::HpFloat;

use crate::{
    consts::{EMPTY_BV_PAIR, characters::skills::levels::Level},
    utils::op::Op,
};

macro_rules! define_skill_registry {
    (
        pub static $registry_name:ident = [
            $(
                $skill_const:ident => $skill_expr:expr
            ),* $(,)?
        ];
    ) => {
        $(
            pub static $skill_const: $crate::consts::characters::skills::ConstSkill = $skill_expr;
        )*

        pub static $registry_name: &[&$crate::consts::characters::skills::ConstSkill] = &[
            $( &$skill_const ),*
        ];
    };
}

pub mod levels;
pub mod registries;

pub const MAX_SKILL_SECONDARIES: usize = 10;

/// Compile-time definition of a character skill and the base values it impacts.
///
/// # Expected Usage & Registry Invariants
/// - Every `ConstSkill` must be registered in [`SKILL_REGISTRY_OF_REGISTRIES`](registries::SKILL_REGISTRY_OF_REGISTRIES)
///   with a globally unique `name`.
/// - At most one `ConstSkill` across all registries may declare a given `base_value_name` as its
///   `master` impact (enforced by unit tests in [`registries`]).
/// - Up to [`MAX_SKILL_SECONDARIES`] (`10`) secondary impacts can be attached via
///   [`ConstSkill::with_secondaries`] (checked at compile time).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstSkill {
    pub name: &'static str,
    pub master: Option<SkillStatImpact>,
    active_secondaries_count: usize,
    secondaries: [SkillStatImpact; MAX_SKILL_SECONDARIES],
}

impl ConstSkill {
    pub const fn new(name: &'static str) -> ConstSkill {
        ConstSkill {
            name,
            master: None,
            active_secondaries_count: 0,
            secondaries: [SkillStatImpact::const_default(); MAX_SKILL_SECONDARIES],
        }
    }
    pub const fn with_secondaries<const N: usize>(
        mut self,
        secondaries: [SkillStatImpact; N],
    ) -> ConstSkill {
        assert!(
            N <= MAX_SKILL_SECONDARIES,
            "Skill has too many secondary skills!"
        );
        let mut i = 0;
        while i < N {
            self.secondaries[i] = secondaries[i];
            i += 1;
        }
        self.active_secondaries_count = N;
        self
    }
    pub const fn with_master(mut self, master: SkillStatImpact) -> ConstSkill {
        self.master = Some(master);
        self
    }
    pub fn secondary_skills(&self) -> &[SkillStatImpact] {
        &self.secondaries[..self.active_secondaries_count]
    }
    pub fn all_impacts(&self) -> Vec<SkillStatImpact> {
        let mut out = if let Some(master) = self.master {
            let mut tmp = Vec::with_capacity(self.active_secondaries_count + 1);
            tmp.push(master);
            tmp
        } else {
            Vec::with_capacity(self.active_secondaries_count)
        };
        out.extend(self.secondary_skills());
        out
    }
}

/// Defines how a [`ConstSkill`] impacts a specific base value (or base value group key).
///
/// # Expected Usage
/// [`SkillStatImpact::new`] initializes `factor` to `0.0` and `op` to [`Op::Add`]; callers
/// defining skills in registries should chain [`SkillStatImpact::with_factor`] (and optionally
/// [`SkillStatImpact::with_op`]).
#[derive(Clone, Copy, Debug)]
pub struct SkillStatImpact {
    pub base_value_name: &'static str,
    pub factor: f32,
    pub op: Op,
}

impl PartialEq for SkillStatImpact {
    fn eq(&self, other: &Self) -> bool {
        if self.base_value_name == other.base_value_name
            && self.factor == other.factor // Apparently deriving eq cannot do this check
            && self.op == other.op
        {
            true
        } else {
            false
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for SkillStatImpact {}

impl SkillStatImpact {
    pub(super) const fn const_default() -> SkillStatImpact {
        SkillStatImpact {
            base_value_name: EMPTY_BV_PAIR.0,
            factor: 0.0,
            op: Op::Add,
        }
    }
    pub const fn new(base_value_name: &'static str) -> SkillStatImpact {
        let mut new_self = Self::const_default();
        new_self.base_value_name = base_value_name;
        new_self
    }
    pub const fn with_factor(mut self, factor: f32) -> SkillStatImpact {
        self.factor = factor;
        self
    }
    pub const fn with_op(mut self, op: Op) -> SkillStatImpact {
        self.op = op;
        self
    }
    /// Computes the `(final_factor, op)` stat impact for a skill given its `aff` (affinity) and
    /// rust-adjusted `effective_level`.
    ///
    /// # Formula
    /// Maps `effective_level` to a tiered `base_level` with non-linear jumps between tiers:
    /// - `Level::Low(0..=9)` -> `1..=10`
    /// - `Level::Medium(0..=9)` -> `12..=21`
    /// - `Level::High(0..=9)` -> `25..=34`
    /// - `Level::Ultra(0..=2)` -> `50, 55, 60`
    ///
    /// Then returns `(self.factor * (base_level * aff), self.op)`.
    pub fn calc_final_stat_impact(&self, aff: &HpFloat, effective_level: Level) -> (HpFloat, Op) {
        let base_level = match effective_level {
            Level::Low(inner) => 1 + inner,          // 1..=10
            Level::Medium(inner) => 12 + inner,      // 12..=21 (jump from 10 -> 12)
            Level::High(inner) => 25 + inner,        // 25..=34 (jump from 21 -> 25)
            Level::Ultra(inner) => 50 + (inner * 5), // 50, 55, 60 (massive jump)
        };
        let level_factor = HpFloat::cast(base_level) * *aff;
        let final_factor: HpFloat = self.factor * level_factor;
        (final_factor, self.op)
    }
}
