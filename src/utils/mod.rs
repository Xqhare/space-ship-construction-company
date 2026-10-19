use fortuna::seed_lengths::FROM_ASCII_RECOMMENDED_MAX;
use nemesis::NemesisResultExt;
use tyche::{RngApi, Tyche};

use crate::error::{SSCCResult, new_startup_error};

pub fn seed_giver() -> SSCCResult<Vec<char>> {
    let mut rng = Tyche::new()
        .map_err(|err| err.to_nemesis_error())
        .add_ctx("Startup error during seed generation with Tyche")?;
    let mut out_buf = Vec::with_capacity(FROM_ASCII_RECOMMENDED_MAX);
    for n in 0..64 {
        out_buf[n] = rng.random_ascii_char().map_err(|e| {
            new_startup_error("Unable to generate seed with Tyche").add_ctx(format!("{:?}", e))
        })?;
    }
    Ok(out_buf)
}

/// `Fortuna` works best when the seed spans all byte values 0..255.
/// Seeds entered by the user (by keyboard) are most likely to be 32..126 (ascii printable characters)
///
/// This function normalises the seed to cover a larger range of bytes.
///
/// # Note
///
/// The seed returned by this function will contain non-printable characters, displaying it will
/// need special care.
///
pub fn seed_normaliser<V: Into<Vec<u8>>>(seed: V) -> Vec<u8> {
    let seed = seed.into();
    let missing_range: Vec<u8> = (128..160).collect();
    let mut out = Vec::with_capacity(seed.len().saturating_mul(3).saturating_add(32));
    for v in seed.iter() {
        out.push(*v % 32);
        out.push(*&v.saturating_add(128));
        out.push(*v);
    }
    for v in missing_range.iter() {
        out.insert(out[*v as usize % out.len()] as usize % out.len(), *v);
    }
    out
}
