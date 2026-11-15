//! Constants for base values.
//!
//! Each base value is a tuple of a name and an optional value, `BaseValueName` and `BaseValueValue` respectively.
//! `BaseValueValue` is a `Option<f32>`.
//!
//! For Values without a value (None), there is no default value that makes sense.
//! A value will need to be calculated for it.

use crate::consts::ConstBVPair;

pub const ALL_CHAR_BASES: [ConstBVPair; 8] = [
    HEALTH,
    MENTAL_HEALTH,
    PAIN,
    MONEY,
    HAPPINESS,
    IQ,
    SOCIABILITY,
    STRENGTH,
];

pub const DEFAULT_CHAR_BASE_VALUE_VALUE: Option<f32> = Some(1000.0);

pub const HEALTH: ConstBVPair = ("health", DEFAULT_CHAR_BASE_VALUE_VALUE);
pub const MENTAL_HEALTH: ConstBVPair = ("mental_health", DEFAULT_CHAR_BASE_VALUE_VALUE);
pub const PAIN: ConstBVPair = ("pain", Some(0.0));
pub const MONEY: ConstBVPair = ("money", Some(0.0));
pub const HAPPINESS: ConstBVPair = ("happiness", DEFAULT_CHAR_BASE_VALUE_VALUE);
pub const SOCIABILITY: ConstBVPair = ("sociability", DEFAULT_CHAR_BASE_VALUE_VALUE);

// Values constructed during char init
pub const IQ: ConstBVPair = ("iq", None);
pub const STRENGTH: ConstBVPair = ("strength", None);
