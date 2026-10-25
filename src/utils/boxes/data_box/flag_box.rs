use std::collections::HashMap;

use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_startup_error;

pub type Flag = String;
/// RefCount for internal use. If it reaches 0, the flag is removed
pub type RefCount = u32;

/// A collection of flags
///
/// If a flag exists, it is considered to be `true`.
///
/// Flags are tracked using a HashMap<Flag, RefCount>.
#[non_exhaustive]
pub struct FlagBox {
    flags: HashMap<Flag, RefCount>,
}

impl XffSerde for FlagBox {
    fn to_xff(&self) -> XffValue {
        xff!(Object::from(self.flags.clone()))
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(flags) = obj.get("flags")
            && let Some(flags) = flags.as_object()
        {
            let mut out_flags = HashMap::new();
            for (key, value) in flags.iter() {
                if let Some(value) = value.as_number()
                    && let Some(value) = value.into_usize()
                {
                    out_flags.insert(key.clone(), value as RefCount);
                } else {
                    return Err(new_startup_error(
                        "Can't convert value to FlagBox, RefCount is not a number.",
                    )
                    .add_ctx(format!("Value: {xff:?}"))
                    .add_ctx(format!("Key=Value[Flag=RefCount]: {key}={value}")));
                }
            }
            Ok(FlagBox { flags: out_flags })
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
            flags: HashMap::new(),
        }
    }
    /// Add a flag to the FlagBox.
    ///
    /// If the flag does not exist, it is added with a RefCount of 1.
    /// If the flag does exist, the RefCount is increased by 1.
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn add_flag<S: Into<String>>(&mut self, flag: S) {
        let flag = flag.into();
        if let Some(inner_flag) = self.flags.get_mut(&flag) {
            *inner_flag += 1;
        } else {
            self.flags.insert(flag, 1);
        }
    }
    /// Remove a flag from the FlagBox.
    ///
    /// Decreases the RefCount of the flag. If it reaches 0, the flag is removed.
    /// Use `delete_flag` to remove the flag completely no matter the RefCount.
    ///
    /// If the flag was not present, nothing happens
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn remove_flag<S: Into<String>>(&mut self, flag: S) {
        let flag = flag.into();
        if let Some(inner_flag) = self.flags.get_mut(&flag) {
            *inner_flag -= 1;
            if *inner_flag == 0 {
                self.flags.remove(&flag);
            }
        }
    }
    /// Immediately remove a flag from the FlagBox.
    ///
    /// If the flag was not present, nothing happens
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn delete_flag<S: Into<String>>(&mut self, flag: S) {
        self.flags.remove(&flag.into());
    }
    /// Check if the FlagBox contains a flag
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn has_flag<S: Into<String>>(&self, flag: S) -> bool {
        self.flags.contains_key(&flag.into())
    }
}
