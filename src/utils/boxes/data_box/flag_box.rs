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
    ///
    /// # Panics
    ///
    /// Panics if the `RefCount` overflows. (`u32::MAX + 1`)
    pub fn add_flag<S: Into<String>>(&mut self, flag: S) {
        let flag = flag.into();
        if let Some(flag_ref_count) = self.flags.get_mut(&flag) {
            *flag_ref_count = *&flag_ref_count.strict_add(1);
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
        if let Some(flag_ref_count) = self.flags.get_mut(&flag) {
            if flag_ref_count == &0 {
                debug_assert!(false, "RefCount should never be 0. Estúpido!");
                // Safety drop for prod
                self.flags.remove(&flag);
            } else {
                *flag_ref_count = *&flag_ref_count.saturating_sub(1);
                if *flag_ref_count == 0 {
                    self.flags.remove(&flag);
                }
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
    /// Merge another FlagBox into this one.
    ///
    /// This will not increase the RefCount of any flags.
    /// If a flag is not present in `self`, but is present in `other`, it will be added to `self` with the RefCount found in `other`.
    pub fn extend(&mut self, other: &FlagBox) {
        for (flag, ref_count) in &other.flags {
            if !self.flags.contains_key(flag) {
                self.flags.insert(flag.to_string(), *ref_count);
            }
        }
    }
    /// Merge another FlagBox into this one.
    ///
    /// This will increase the RefCount of any flags in `self` if they are also present in `other`.
    /// If a flag is not present in `self`, but is present in `other`, it will be added to `self` with the RefCount found in `other`.
    pub fn extend_and_inc_ref(&mut self, other: &FlagBox) {
        for (flag, ref_count) in &other.flags {
            if let Some(inner_ref_count) = self.flags.get_mut(flag) {
                *inner_ref_count += ref_count;
            } else {
                self.flags.insert(flag.to_string(), *ref_count);
            }
        }
    }
}
