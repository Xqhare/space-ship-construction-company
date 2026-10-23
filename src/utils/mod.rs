use fortuna::seed_lengths::FROM_ASCII_RECOMMENDED_MAX;
use horae::Utc;
use nemesis::NemesisResultExt;
use tyche::{RngApi, Tyche};

use crate::error::{SSCCResult, new_startup_error};
pub mod boxes;

pub fn start_date() -> Utc {
    Utc::from_ymd_hms(2500, 1, 1, 0, 0, 0)
}

/// Don't forget to chain with `fortuna::helper::normalise_ascii_seed`
pub fn seed_giver() -> SSCCResult<Vec<char>> {
    let mut rng = Tyche::new()
        .map_err(|err| err.to_nemesis_error())
        .add_ctx("Startup error during seed generation with Tyche")?;
    let mut out_buf = vec!['c'; FROM_ASCII_RECOMMENDED_MAX];
    for n in 0..out_buf.len() {
        out_buf[n] = rng.random_ascii_char().map_err(|e| {
            new_startup_error("Unable to generate seed with Tyche").add_ctx(format!("{:?}", e))
        })?;
    }
    Ok(out_buf)
}
