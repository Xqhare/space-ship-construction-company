//! Constants for base values.
//!
//! Each base value is a tuple of a name and an optional value, `BaseValueName` and `BaseValueValue` respectively.
//! `BaseValueValue` is a `Option<f32>`.
//!
//! For Values without a value (None), there is no default value that makes sense.
//! A value will need to be calculated for it.

use crate::consts::ConstBVPair;

pub const ALL_CHAR_BASES: [ConstBVPair; 13] = [
    HEALTH,
    MENTAL_HEALTH,
    PAIN,
    MONEY,
    HAPPINESS,
    ENVIRONMENT,
    IQ,
    SOCIABILITY,
    FULFILMENT,
    COMFORT,
    SOCIAL,
    STRESS,
    FUN,
];

pub const HEALTH: ConstBVPair = ("health", Some(1000.0));
pub const MENTAL_HEALTH: ConstBVPair = ("mental_health", Some(100.0));
pub const PAIN: ConstBVPair = ("pain", Some(0.0));
pub const MONEY: ConstBVPair = ("money", None);
pub const HAPPINESS: ConstBVPair = ("happiness", Some(100.0));
pub const ENVIRONMENT: ConstBVPair = ("environment", None);
pub const IQ: ConstBVPair = ("iq", None);
pub const SOCIABILITY: ConstBVPair = ("sociability", None);
pub const FULFILMENT: ConstBVPair = ("fulfilment", None);
pub const COMFORT: ConstBVPair = ("comfort", None);
pub const SOCIAL: ConstBVPair = ("social", None);
pub const STRESS: ConstBVPair = ("stress", None);
pub const FUN: ConstBVPair = ("fun", None);
