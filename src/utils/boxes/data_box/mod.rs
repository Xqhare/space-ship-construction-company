pub mod flag_box;
pub mod stat_box;
use aequa::{Object, XffSerde, XffValue, xff};
use nemesis::NemesisError;

use crate::{
    error::new_startup_error,
    utils::boxes::data_box::{flag_box::FlagBox, stat_box::StatBox},
};

/// Generalised storage of data for entities
pub struct DataBox {
    stats: StatBox,
    flags: FlagBox,
}

impl XffSerde for DataBox {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("flags", self.flags.to_xff());
        obj.insert("stats", self.stats.to_xff());
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(flags) = obj.get("flags")
            && let Some(stats) = obj.get("stats")
        {
            Ok(DataBox {
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
}

impl DataBox {
    /// Create a new, completely empty, DataStore
    pub fn new() -> DataBox {
        DataBox {
            stats: StatBox::new(),
            flags: FlagBox::new(),
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
    /// Returns a mutable reference to the FlagBox
    pub fn get_flag_box_mut(&mut self) -> &mut FlagBox {
        &mut self.flags
    }
    /// Returns a reference to the FlagBox
    pub fn get_flag_box(&self) -> &FlagBox {
        &self.flags
    }
    /// Merge another DataBox into this one
    ///
    /// For a detailed description of how this works, see [`StatBox::extend`] and [`FlagBox::extend`]
    pub fn extend(&mut self, other: &DataBox) {
        self.stats.extend(&other.stats);
        self.flags.extend(&other.flags);
    }
}
