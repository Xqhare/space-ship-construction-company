use athena::float::HpFloat;
use tyche::RngApi;

use crate::{error::SSCCResult, utils::Utils};

pub fn rand_bool(utils: &mut Utils) -> SSCCResult<bool> {
    utils
        .rng
        .random_bool()
        .map_err(|err| err.to_nemesis_error())
}
pub fn random_from_range_inclusive(
    utils: &mut Utils,
    min: usize,
    max: usize,
) -> SSCCResult<HpFloat> {
    Ok(HpFloat::new(
        utils
            .rng
            .random_from_range_inclusive(min, max)
            .map_err(|err| err.to_nemesis_error())? as i128,
        0,
    ))
}
