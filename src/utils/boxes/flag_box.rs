use std::collections::HashSet;

use aequa::{Array, XffSerde, XffValue, xff};

use crate::error::new_startup_error;

pub type Flag = String;

/// A collection of flags
///
/// If a flag exists, it is considered to be `true`.
#[non_exhaustive]
pub struct FlagBox {
    flags: HashSet<Flag>,
}

impl XffSerde for FlagBox {
    fn to_xff(&self) -> XffValue {
        let mut ary = Array::new();
        for flag in &self.flags {
            ary.push(xff!(flag));
        }
        xff!(ary)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(ary) = xff.as_array() {
            let mut flags = HashSet::new();
            for flag in ary {
                if let Some(flag) = flag.into_string() {
                    flags.insert(flag);
                } else {
                    return Err(new_startup_error(
                        "Can't convert value to a flag in FlagBox, value not a string.",
                    )
                    .add_ctx(format!("Flag value: {:?}", flag))
                    .add_ctx(format!("Parent value: {:?}", xff)));
                }
            }
            Ok(FlagBox { flags })
        } else {
            Err(
                new_startup_error("Can't convert value to FlagBox, not an array.")
                    .add_ctx(format!("Value: {:?}", xff)),
            )
        }
    }
}

impl FlagBox {
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
