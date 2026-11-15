use crate::consts::{
    base_values::{
        REGISTRY_OF_BASE_VALUE_REGISTRIES, SOC_BONUS_ALL,
        science::{SCI_BONUS_ALL_FIELDS, SCI_REGISTRY_OF_REGISTRIES},
    },
    characters::base_values::SOCIABILITY,
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

pub fn get_bvp_by_bv_name(bvp_name: &'static str) -> Option<&'static ConstBVPair> {
    REGISTRY_OF_BASE_VALUE_REGISTRIES
        .iter()
        .find_map(|reg| reg.iter().find(|entry| entry.0 == bvp_name))
}

/// Normalise many bvp_names
pub fn normalise_bvp_names<'a>(bvp_names: &'a [&'static str]) -> Vec<&'static str> {
    bvp_names
        .iter()
        .map(|entry| normalise_bvp_name(entry))
        .flatten()
        .collect()
}

/// Normalise one bvp_name
pub fn normalise_bvp_name(bvp_name: &'static str) -> Vec<&'static str> {
    if bvp_name == SCI_BONUS_ALL_FIELDS.0 {
        SCI_REGISTRY_OF_REGISTRIES
            .iter()
            .flat_map(|reg| reg.iter().map(|field| field.name))
            .collect()
    } else if bvp_name == SOC_BONUS_ALL.0 {
        vec![SOCIABILITY.0]
    } else {
        vec![bvp_name]
    }
}
