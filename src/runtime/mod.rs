use std::{path::Path, time::Duration};

use aequa::{Object, XffSerde, XffValue, xff};
use athena::slot_map::{SlotMap, SlotMapIndex};

use crate::{
    error::{SSCCResult, new_startup_error},
    modules::event::{
        event_queue::{EventQueue, queue_item::QueueItem},
        event_store::EventStore,
    },
    utils::{
        Utils,
        boxes::{data_box::DataBox, super_box::SuperBox},
    },
};

pub type DataBoxSlotId = SlotMapIndex;

/// The Space Ship Construction Company 'Runtime' object
pub struct Runtime {
    pub utils: Utils,
    pub data_box_slot_map: SlotMap<DataBox>,
    pub super_box: SuperBox,
    pub event_queue: EventQueue,
    pub event_store: EventStore,
}

impl Runtime {
    pub fn increment_time(&mut self, desired_duration: Duration) -> Option<Duration> {
        if let Some(event_queue_output) = self
            .event_queue
            .get_next_event(self.super_box.world_box().now_utc(), desired_duration)
        {
            todo!("inc world time & stuff");
            Some(self.handle_event(event_queue_output))
        } else {
            None
        }
    }
    fn handle_event(&mut self, event_queue_output: (Vec<QueueItem>, Duration)) -> Duration {
        let (queue_items, duration) = event_queue_output;
        todo!("handle event")
    }
    pub fn new_seeded<V: Into<Vec<char>>, P: AsRef<Path>>(
        init_world_seed: V,
        word_smith_path: P,
        all_events_path: P,
    ) -> SSCCResult<Runtime> {
        let mut slot_map: SlotMap<DataBox> = SlotMap::with_capacity(64);
        let mut super_box = SuperBox::new(
            slot_map.insert(DataBox::new()),
            slot_map.insert(DataBox::new()),
            init_world_seed,
        );
        let utils = Utils::new(
            word_smith_path,
            super_box.world_box().world_seed_normalised().clone(),
        )?;
        let utc_now = super_box.world_box().now_utc();
        let event_queue = EventQueue::new(utc_now);
        let event_store = EventStore::init_new(all_events_path)?;
        Ok(Runtime {
            utils,
            data_box_slot_map: slot_map,
            super_box,
            event_queue,
            event_store,
        })
    }
}
