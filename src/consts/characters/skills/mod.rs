use crate::{consts::EMPTY_BV_PAIR, utils::op::Op};

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

#[derive(Clone, Copy, Debug)]
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
}

#[derive(Clone, Copy, Debug)]
pub struct SkillStatImpact {
    pub base_value_name: &'static str,
    pub factor: f32,
    pub op: Op,
}

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
}
