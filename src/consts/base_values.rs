//! Constants for base values.
//!
//! Each base value is a tuple of a name and an optional value, `BaseValueName` and `BaseValueValue` respectivly.
//! `BaseValueValue` is a `Option<f32>`.

pub type ConstBVPair<'a> = (&'a str, Option<f32>);

pub const TEST_BASE_VALUE_PAIR: ConstBVPair = ("test_base_value", Some(1.0));
pub const HEALTH: ConstBVPair = ("health", None);
pub const EMPLOYEES: ConstBVPair = ("employees", None);
pub const MONEY: ConstBVPair = ("employees", None);
pub const RANGE: ConstBVPair = ("employees", None);
