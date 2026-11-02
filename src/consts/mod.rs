pub mod base_values;
pub mod characters;
pub mod flags;
pub mod science;

pub type ConstBVPair<'a> = (&'a str, Option<f32>);

pub const EMPTY_BV_PAIR: ConstBVPair = ("", None);

pub mod durations {
    use std::time::Duration;

    pub const DAY_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24);
    pub const WEEK_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    pub const FORTNIGHT_DUR_SECS: Duration = Duration::from_secs(60 * 60 * 24 * 14);
}
