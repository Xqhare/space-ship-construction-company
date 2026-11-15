use aequa::{Object, XffSerde, XffValue, xff};

use crate::{
    consts::characters::skills::{levels::Level, registries::get_skill_from_registry_by_name},
    error::new_xff_serde_error,
    modules::character::skill::CharacterSkill,
};

const SKILL_FIELD: &str = "skill";
const AFFINITY_FIELD: &str = "aff";
const POINTS_FIELD: &str = "points";
const LEVEL_FIELD: &str = "level";
const UNUSED_COUNTER_FIELD: &str = "unused";
const LOST_RUST_THIS_SIM_TICK_FIELD: &str = "lost_rust";
const RUST_FIELD: &str = "rust";
const MAX_RUST_FIELD: &str = "max_rust";

impl XffSerde for CharacterSkill {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert(SKILL_FIELD, self.skill.name);
        obj.insert(AFFINITY_FIELD, self.affinity);
        obj.insert(POINTS_FIELD, self.points);
        obj.insert(LEVEL_FIELD, self.level.to_ordinal());
        obj.insert(UNUSED_COUNTER_FIELD, self.unused_counter);
        obj.insert(LOST_RUST_THIS_SIM_TICK_FIELD, self.lost_rust_this_sim_tick);
        obj.insert(RUST_FIELD, self.rust);
        obj.insert(MAX_RUST_FIELD, self.max_rust);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(skill) = obj.get(SKILL_FIELD)
                && let Some(skill) = skill.as_string()
                && let Some(aff) = obj.get(AFFINITY_FIELD)
                && let Some(affinity) = aff.into_hp_float()
                && let Some(points) = obj.get(POINTS_FIELD)
                && let Some(points) = points.into_hp_float()
                && let Some(level) = obj.get(LEVEL_FIELD)
                && let Some(level) = level.into_number()
                && let Some(level_ord) = level.into_usize()
                && let Some(unused) = obj.get(UNUSED_COUNTER_FIELD)
                && let Some(unused) = unused.into_number()
                && let Some(unused) = unused.into_usize()
                && let Some(lost_rust) = obj.get(LOST_RUST_THIS_SIM_TICK_FIELD)
                && let Some(lost_rust_this_sim_tick) = lost_rust.into_boolean()
                && let Some(rust) = obj.get(LEVEL_FIELD)
                && let Some(rust) = rust.into_number()
                && let Some(rust) = rust.into_usize()
                && let Some(max_rust) = obj.get(LEVEL_FIELD)
                && let Some(max_rust) = max_rust.into_number()
                && let Some(max_rust) = max_rust.into_usize()
            {
                let skill = get_skill_from_registry_by_name(skill).ok_or(
                    new_xff_serde_error(&format!(
                        "Cannot parse '{}' as a valid ConstSkill.",
                        skill.to_string(),
                    ))
                    .add_ctx(format!("XffValue: {xff:?}")),
                )?;
                let level = Level::from_ordinal(level_ord as u8);
                let unused_counter = unused as u8;
                let rust = rust as u8;
                let max_rust = max_rust as u8;
                Ok(CharacterSkill {
                    skill,
                    affinity,
                    points,
                    level,
                    unused_counter,
                    lost_rust_this_sim_tick,
                    rust,
                    max_rust,
                })
            } else {
                Err(
                    new_xff_serde_error("Cannot parse CharacterName; Required fields missing!")
                        .add_ctx(format!("Value: '{xff:?}'")),
                )
            }
        } else {
            Err(
                new_xff_serde_error("Cannot parse CharacterName; XffValue not an object!")
                    .add_ctx(format!("{xff:?}")),
            )
        }
    }
}
