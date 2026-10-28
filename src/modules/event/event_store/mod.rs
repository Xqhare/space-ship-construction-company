use std::{
    collections::HashMap,
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    time::Duration,
};

use aequa::{Array, Data, Object, XffSerde, XffValue, xff};
use horae::Utc;

use crate::{
    error::{SSCCResult, new_xff_serde_error},
    modules::event::{Event, EventChoiceId, EventId, event_id_pool::EventIdPools},
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
    all_events_path: PathBuf,
    event_pool: EventIdPools,
}

impl XffSerde for EventStore {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
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
                obj.insert("utc", utc.to_xff());
                store.push(xff!(obj));
            }
            store
        };
        let all_events_path = {
            XffValue::Data(Data::from(
                self.all_events_path.as_os_str().as_bytes().to_vec(),
            ))
        };
        obj.insert("all_events_path", all_events_path);
        obj.insert("triggered_and_blocked", triggered_and_blocked);
        obj.insert("triggered_and_tmp_blocked", triggered_and_tmp_blocked);
        obj.insert("triggered", triggered);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(all_events_path) = obj.get("all_events_path")
            && let Some(all_events_path) = all_events_path.as_data()
            && let Some(triggered_and_blocked) = obj.get("triggered_and_blocked")
            && let Some(triggered_and_blocked) = triggered_and_blocked.as_array()
            && let Some(triggered_and_tmp_blocked) = obj.get("triggered_and_tmp_blocked")
            && let Some(triggered_and_tmp_blocked) = triggered_and_tmp_blocked.as_array()
            && let Some(triggered) = obj.get("triggered")
            && let Some(triggered) = triggered.as_array()
        {
            let all_events_path = PathBuf::from(OsStr::from_bytes(all_events_path.as_slice()));
            let (store, event_pool) = load_events_fs(&all_events_path)?;
            let triggered_and_blocked = {
                let mut store = Vec::new();
                for event_id in triggered_and_blocked.iter() {
                    if let Some(event_id) = event_id.as_number()
                        && let Some(event_id) = event_id.into_usize()
                    {
                        store.push(event_id as u32);
                    }
                }
                store
            };
            let triggered_and_tmp_blocked = {
                let mut store = HashMap::new();
                for inner_obj in triggered_and_tmp_blocked.iter() {
                    if let Some(obj) = inner_obj.as_object()
                        && let Some(event_id) = obj.get("event_id")
                        && let Some(event_id) = event_id.as_number()
                        && let Some(event_id) = event_id.into_usize()
                        && let Some(tmp_blocked_event) = obj.get("tmp_blocked_event")
                    {
                        let tmp_blocked_event = TmpBlockedEvent::from_xff(tmp_blocked_event)?;
                        store.insert(event_id as u32, tmp_blocked_event);
                    }
                }
                store
            };
            let triggered = {
                let mut store = Vec::new();
                for inner_obj in triggered.iter() {
                    if let Some(obj) = inner_obj.as_object()
                        && let Some(event_id) = obj.get("event_id")
                        && let Some(event_id) = event_id.as_number()
                        && let Some(event_id) = event_id.into_usize()
                        && let Some(event_choice_id) = obj.get("event_choice_id")
                        && let Some(event_choice_id) = event_choice_id.as_number()
                        && let Some(event_choice_id) = event_choice_id.into_usize()
                        && let Some(utc) = obj.get("utc")
                    {
                        let utc = Utc::from_xff(utc)?;
                        store.push((event_id as u32, event_choice_id as u8, utc));
                    }
                }
                store
            };
            Ok(EventStore {
                store,
                triggered_and_blocked,
                triggered_and_tmp_blocked,
                triggered,
                all_events_path,
                event_pool,
            })
        } else {
            Err(new_xff_serde_error("Unable to load EventStore from XFF"))
        }
    }
}

impl EventStore {
    pub fn init_new<P: AsRef<Path>>(all_events_path: P) -> SSCCResult<EventStore> {
        let all_events_path = all_events_path.as_ref();
        let (store, event_pool) = load_events_fs(&all_events_path)?;
        Ok(EventStore {
            store,
            triggered_and_blocked: Vec::new(),
            triggered_and_tmp_blocked: HashMap::new(),
            triggered: Vec::new(),
            all_events_path: all_events_path.to_path_buf(),
            event_pool,
        })
    }
}

fn load_events_fs<P: AsRef<Path>>(
    all_events_path: P,
) -> SSCCResult<(HashMap<EventId, Event>, EventIdPools)> {
    let mut out = HashMap::new();
    let mut event_pool = EventIdPools::new();
    // TODO: Load events from the file system & populate the pool
    Ok((out, event_pool))
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct TmpBlockedEvent {
    event_id: EventId,
    blocked_until: Utc,
}

impl XffSerde for TmpBlockedEvent {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("event_id", self.event_id);
        obj.insert("blocked_until", self.blocked_until.to_xff());
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(event_id) = obj.get("event_id")
            && let Some(blocked_until) = obj.get("blocked_until")
        {
            if let Some(event_id) = event_id.as_number()
                && let Some(event_id) = event_id.into_usize()
            {
                let blocked_until = Utc::from_xff(blocked_until)?;
                Ok(TmpBlockedEvent {
                    event_id: event_id as u32,
                    blocked_until,
                })
            } else {
                Err(
                    new_xff_serde_error("Unable to load TmpBlockedEvent from XFF")
                        .add_ctx(format!("Value: {xff:?}")),
                )
            }
        } else {
            Err(
                new_xff_serde_error("Unable to load TmpBlockedEvent from XFF, not an object")
                    .add_ctx(format!("Value: {xff:?}")),
            )
        }
    }
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
}
