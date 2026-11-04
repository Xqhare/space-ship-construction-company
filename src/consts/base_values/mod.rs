//! Constants for base values.
//!
//! Each base value is a tuple of a name and an optional value, `BaseValueName` and `BaseValueValue` respectivly.
//! `BaseValueValue` is a `Option<f32>`.

pub mod science;

use crate::consts::ConstBVPair;

pub const TEST_BASE_VALUE_PAIR: ConstBVPair = ("test_base_value", Some(1.0));
pub const HEALTH: ConstBVPair = ("health", None);
pub const EMPLOYEES: ConstBVPair = ("employees", None);
pub const MONEY: ConstBVPair = ("money", None);
pub const RANGE: ConstBVPair = ("range", None);

pub const SOC_BONUS_ALL: ConstBVPair = ("soc_bonus_all", None);
