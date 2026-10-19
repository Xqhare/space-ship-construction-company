use std::collections::HashSet;

use aequa::{Array, XffValue, xff};

use crate::error::{SSCCResult, new_startup_error};

/// A collection of flags
///
/// If a flag exists, it is considered to be `true`.
#[non_exhaustive]
pub struct FlagBox {
    flags: HashSet<String>,
}

impl FlagBox {
    pub fn to_xff(&self) -> XffValue {
        let mut ary = Array::new();
        for flag in &self.flags {
            ary.push(xff!(flag));
        }
        xff!(ary)
    }
    pub fn from_xff(value: &XffValue) -> SSCCResult<FlagBox> {
        if let Some(ary) = value.as_array() {
            let mut flags = HashSet::new();
            for flag in ary {
                if let Some(flag) = flag.into_string() {
                    flags.insert(flag);
                } else {
                    return Err(new_startup_error(
                        "Can't convert value to a flag in FlagBox, value not a string.",
                    )
                    .add_ctx(format!("Flag value: {:?}", flag))
                    .add_ctx(format!("Parent value: {:?}", value)));
                }
            }
            Ok(FlagBox { flags })
        } else {
            Err(
                new_startup_error("Can't convert value to FlagBox, not an array.")
                    .add_ctx(format!("Value: {:?}", value)),
            )
        }
    }
    /// Create a new, completely empty, FlagBox
    pub fn new() -> FlagBox {
        FlagBox {
            flags: HashSet::new(),
        }
    }
    /// Add a flag to the FlagBox
    ///
    /// If the flag already exists, nothing happens and the argument is dropped
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn add_flag<S: Into<String>>(&mut self, flag: S) {
        self.flags.insert(flag.into());
    }
    /// Remove a flag from the FlagBox
    ///
    /// If the flag was not present, nothing happens
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn remove_flag<S: Into<String>>(&mut self, flag: S) {
        self.flags.remove(&flag.into());
    }
    /// Check if the FlagBox contains a flag
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn has_flag<S: Into<String>>(&self, flag: S) -> bool {
        self.flags.contains(&flag.into())
    }
}
