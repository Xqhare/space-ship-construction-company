pub mod base_values;
pub mod flags;
pub mod durations {
    use std::time::Duration;

    pub const DAY_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24);
    pub const WEEK_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    pub const FORTNIGHT_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 14);
}
