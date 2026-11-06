use crate::consts::characters::flags::suicide::{SUICIDE_NEVER_OFF_SITE, SUICIDE_NEVER_ON_SITE};

pub mod capabilities;
pub const TEST_FLAG: &str = "test_flag";
/// For internal use only.
///
/// E.g. As a stand in for Builders that need an allocated String.
///
/// If used, ensure that this cannot be encountered by users.
///
/// Example usage can be found within [`CapabilityBuilder`]
pub const INTERNAL_NONE: &str = "internal_none";

pub fn get_conflicting_flags(flag: &str) -> Vec<&'static str> {
    // If more than 4 conflicting flags exist for any flag, increase the cap
    let mut out = Vec::with_capacity(4);
    match flag {
        SUICIDE_NEVER_ON_SITE => out.push(SUICIDE_NEVER_OFF_SITE),
        SUICIDE_NEVER_OFF_SITE => out.push(SUICIDE_NEVER_ON_SITE),
        _ => {}
    }
    out
}
