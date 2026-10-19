use aequa::{Object, XffValue, xff};

use crate::{
    error::{SSCCResult, new_startup_error},
    modules::{flag_box::FlagBox, stat_box::StatBox},
};

/// Generalised storage for player or world data
pub struct DataStore {
    stats: StatBox,
    flags: FlagBox,
}

impl DataStore {
    /// Create a new, completely empty, DataStore
    pub fn new() -> DataStore {
        DataStore {
            stats: StatBox::new(),
            flags: FlagBox::new(),
        }
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("flags", self.flags.to_xff());
        obj.insert("stats", self.stats.to_xff());
        xff!(obj)
    }
    pub fn from_xff(xff: &XffValue) -> SSCCResult<DataStore> {
        if let Some(obj) = xff.as_object()
            && let Some(flags) = obj.get("flags")
            && let Some(stats) = obj.get("stats")
        {
            Ok(DataStore {
                stats: StatBox::from_xff(stats)?,
                flags: FlagBox::from_xff(flags)?,
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to DataStore, not an object or doesn't have expected keys.",
            )
            .add_ctx(format!("Value: {:?}", xff)))
        }
    }
    /// Checks if a flag of that name exists or if a base value of that name exists
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn is_tracking<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        self.flags.has_flag(&name) || self.stats.is_tracking(name)
    }
    /// Returns a mutable reference to the StatBox
    pub fn get_stat_box_mut(&mut self) -> &mut StatBox {
        &mut self.stats
    }
    /// Returns a reference to the StatBox
    pub fn get_stat_box(&self) -> &StatBox {
        &self.stats
    }
    /// Add a flag to the FlagBox
    ///
    /// If the flag already exists, nothing happens and the argument is dropped
    ///
    /// Flags may only exist once. This is enforced by the FlagBox
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn add_flag<S: Into<String>>(&mut self, flag: S) {
        self.flags.add_flag(flag);
    }
    /// Remove a flag from the FlagBox
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn remove_flag<S: Into<String>>(&mut self, flag: S) {
        self.flags.remove_flag(flag);
    }
    /// Check if the FlagBox contains a flag
    ///
    /// If a flag exists, it is considered to be `true`.
    pub fn has_flag<S: Into<String>>(&self, flag: S) -> bool {
        self.flags.has_flag(flag)
    }
}
