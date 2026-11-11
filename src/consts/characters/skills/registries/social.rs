use crate::consts::characters::{
    base_values::SOCIABILITY,
    skills::{ConstSkill, SkillStatImpact},
};

define_skill_registry! {
    pub static SKILL_SOCIAL_REGISTRY = [
        SKILL_SOCIAL_LYING => ConstSkill::new("Lying").with_secondaries([SkillStatImpact::new(SOCIABILITY.0).with_factor(1.0)]),
        SKILL_SOCIAL_MANIPULATION => ConstSkill::new("Manipulation").with_secondaries([SkillStatImpact::new(SOCIABILITY.0).with_factor(1.0)]),
        SKILL_SOCIAL_JUDGE_OF_INTENT => ConstSkill::new("Judge of intent").with_secondaries([SkillStatImpact::new(SOCIABILITY.0).with_factor(1.0)]),
    ];
}
