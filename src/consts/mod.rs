use crate::consts::{
    base_values::{
        SOC_BONUS_ALL,
        science::{SCI_BONUS_ALL_FIELDS, SCI_REGISTRY_OF_REGISTRIES},
    },
    characters::skills::registries::social::SKILL_SOCIAL_REGISTRY,
};

pub mod base_values;
pub mod characters;
pub mod flags;

pub type ConstBVPair = (&'static str, Option<f32>);

pub const EMPTY_BV_PAIR: ConstBVPair = ("", None);

pub mod durations {
    use std::time::Duration;

    pub const DAY_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24);
    pub const WEEK_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    pub const FORTNIGHT_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 14);
}

pub fn normalise_bvp_name(bvp_name: &'static str) -> Vec<&'static str> {
    if bvp_name == SCI_BONUS_ALL_FIELDS.0 {
        SCI_REGISTRY_OF_REGISTRIES
            .iter()
            .flat_map(|reg| reg.iter().map(|field| field.name))
            .collect()
    } else if bvp_name == SOC_BONUS_ALL.0 {
        SKILL_SOCIAL_REGISTRY
            .iter()
            .map(|entry| entry.name)
            .collect()
    } else {
        vec![bvp_name]
    }
}
