use std::{collections::HashMap, path::Path, time::Duration};

use aequa::{Array, Object, XffValue, xff};
use horae::Utc;

use crate::{
    error::SSCCResult,
    modules::event::{Event, EventChoiceId, EventId, event_pool::EventPools},
};

#[non_exhaustive]
pub struct EventStore {
    /// Store of all events
    store: HashMap<EventId, Event>,
    /// Store of all events that have been triggered and thus are blocked from happening again
    triggered_and_blocked: Vec<EventId>,
    /// Store of all events that have been triggered and thus are blocked from happening again
    /// for a specific duration
    triggered_and_tmp_blocked: HashMap<EventId, TmpBlockedEvent>,
    /// Store of all events that have ever been triggered. May contain duplicate `EventId`s
    /// if an event is triggered multiple times.
    ///
    /// Format: `[(EventId, EventChoiceId, Utc)]`
    /// Where `EventChoiceId` is the ID of the event choice that was picked and `Utc` is the
    /// time that the event was triggered
    triggered: Vec<(EventId, EventChoiceId, Utc)>,
    event_pool: EventPools,
}

impl EventStore {
    pub fn new<P: AsRef<Path>>(all_events_path: P) -> SSCCResult<EventStore> {
        let store = load_events_fs(all_events_path)?;
        Ok(EventStore {
            store,
            triggered_and_blocked: Vec::new(),
            triggered_and_tmp_blocked: HashMap::new(),
            triggered: Vec::new(),
            event_pool: EventPools::new(),
        })
    }
    pub fn from_xff(xff: &XffValue) -> SSCCResult<EventStore> {
        todo!()
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        let store = {
            let mut store = Object::new();
            for (event_id, event) in self.store.iter() {
                store.insert(&event_id.to_string(), event.to_xff());
            }
            store
        };
        let triggered_and_blocked = {
            let mut store = Array::new();
            for event_id in self.triggered_and_blocked.iter() {
                store.push(*event_id);
            }
            store
        };
        let triggered_and_tmp_blocked = {
            let mut store = Array::new();
            for (event_id, tmp_blocked_event) in self.triggered_and_tmp_blocked.iter() {
                let mut inner_obj = Object::new();
                inner_obj.insert("event_id", *event_id);
                inner_obj.insert("tmp_blocked_event", tmp_blocked_event.to_xff());
                store.push(xff!(inner_obj));
            }
            store
        };
        let triggered = {
            let mut store = Array::new();
            for (event_id, event_choice_id, utc) in self.triggered.iter() {
                let mut obj = Object::new();
                obj.insert("event_id", *event_id);
                obj.insert("event_choice_id", *event_choice_id);
                obj.insert("utc", utc.to_xffvalue());
                store.push(xff!(obj));
            }
            store
        };
        obj.insert("store", store);
        obj.insert("triggered_and_blocked", triggered_and_blocked);
        obj.insert("triggered_and_tmp_blocked", triggered_and_tmp_blocked);
        obj.insert("triggered", triggered);
        xff!(obj)
    }
}

fn load_events_fs<P: AsRef<Path>>(all_events_path: P) -> SSCCResult<HashMap<EventId, Event>> {
    let mut out = HashMap::new();
    // TODO: Load events from the file system
    Ok(out)
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct TmpBlockedEvent {
    event_id: EventId,
    blocked_until: Utc,
}

impl TmpBlockedEvent {
    pub fn new(event_id: EventId, now: Utc, block_duration: Duration) -> TmpBlockedEvent {
        TmpBlockedEvent {
            event_id,
            blocked_until: now + block_duration,
        }
    }
    pub fn is_blocked(&self, now: Utc) -> bool {
        now < self.blocked_until
    }
    pub fn get_event_id(&self) -> EventId {
        self.event_id
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("event_id", self.event_id);
        obj.insert("blocked_until", self.blocked_until.to_xffvalue());
        xff!(obj)
    }
    pub fn from_xff(value: &XffValue) -> Result<TmpBlockedEvent, ()> {
        value.try_into()
    }
}

impl TryInto<TmpBlockedEvent> for &XffValue {
    type Error = ();
    fn try_into(self) -> Result<TmpBlockedEvent, Self::Error> {
        if let Some(obj) = self.as_object()
            && let Some(event_id) = obj.get("event_id")
            && let Some(blocked_until) = obj.get("blocked_until")
        {
            if let Some(event_id) = event_id.as_number()
                && let Some(event_id) = event_id.into_usize()
                && let Some(blocked_until) = Utc::from_xffvalue(blocked_until.clone())
            {
                Ok(TmpBlockedEvent {
                    event_id: event_id as u32,
                    blocked_until,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
