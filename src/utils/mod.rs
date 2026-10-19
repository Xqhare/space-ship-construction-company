use nemesis::NemesisResultExt;
use tyche::{RngApi, Tyche};

use crate::error::{SSCCError, SSCCResult, new_startup_error};

pub fn seed_giver() -> SSCCResult<Vec<char>> {
    let mut rng = Tyche::new()
        .map_err(|e| SSCCError::Io(e))
        .add_ctx("Startup error during seed generation with Tyche")?;
    let mut out_buf = Vec::with_capacity(64);
    for n in 0..64 {
        out_buf[n] = rng.random_ascii_char().map_err(|e| {
            new_startup_error("Unable to generate seed with Tyche").add_ctx(format!("{:?}", e))
        })?;
    }
    Ok(out_buf)
}
