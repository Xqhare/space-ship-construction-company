use std::{path::Path, time::Duration};

use aequa::{Object, XffValue, xff};

use crate::{
    error::{SSCCResult, new_startup_error},
    modules::{
        event::{event_queue::EventQueue, event_store::EventStore},
        super_store::SuperStore,
    },
};

pub struct SSCC {
    pub super_store: SuperStore,
    pub event_queue: EventQueue,
    pub event_store: EventStore,
}

impl SSCC {
    pub fn increment_time(&mut self, duration: Duration) -> Duration {}
    pub fn new<P: AsRef<Path>>(word_smith_path: P, all_events_path: P) -> SSCCResult<SSCC> {
        let mut super_store = SuperStore::new(word_smith_path)?;
        let event_queue = EventQueue::new(super_store.world_time().clone());
        let event_store = EventStore::init_new(all_events_path)?;
        Ok(SSCC {
            super_store,
            event_queue,
            event_store,
        })
    }
    pub fn new_seeded<V: Into<Vec<char>>, P: AsRef<Path>>(
        seed: V,
        word_smith_path: P,
    ) -> SSCCResult<SSCC> {
        let seed = seed.into().iter().map(|c| *c as u8).collect::<Vec<u8>>();
        Ok(SSCC {
            super_store: SuperStore::new_seeded(seed, word_smith_path)?,
        })
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("super_store", self.super_store.to_xff());
        xff!(obj)
    }
    pub fn from_xff<P: AsRef<Path>>(xff: &XffValue, word_smith_path: P) -> SSCCResult<SSCC> {
        if let Some(obj) = xff.as_object()
            && let Some(super_store) = obj.get("super_store")
        {
            Ok(SSCC {
                super_store: SuperStore::from_xff(super_store, word_smith_path)?,
            })
        } else {
            Err(new_startup_error(
                "Can't convert value to SSCC, not an object or doesn't have expected keys.",
            )
            .add_ctx(format!("Value: {:?}", xff)))
        }
    }
}
