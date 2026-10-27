use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;

use crate::{
    error::new_xff_serde_error,
    modules::event::event_queue::{queue_event::QueueEvent, tick::Tick},
    runtime::DataBoxSlotId,
};

/// Helper enum for the event queue
///
/// Contains either an event or a tick
pub enum QueueItem {
    /// An event for the user / player to interact with
    /// Some need to interrupt the passing of time, others don't
    Event(QueueEvent),
    /// A tick (day, week, month, year, etc) for internal use
    Tick(Tick),
    Modifier(QueueModifier),
}

pub struct QueueModifier {
    pub time: Utc,
    pub data_box_slot_id: DataBoxSlotId,
}

impl XffSerde for QueueModifier {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xff());
        obj.insert("data_box_slot_id", self.data_box_slot_id.to_xff());
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(time) = obj.get("time")
            && let Some(data_box_slot_id) = obj.get("data_box_slot_id")
        {
            let time = Utc::from_xff(time)?;
            let data_box_slot_id = DataBoxSlotId::from_xff(data_box_slot_id)?;
            Ok(QueueModifier {
                time,
                data_box_slot_id,
            })
        } else {
            Err(new_xff_serde_error("Unable to load QueueModifier from XFF")
                .add_ctx(format!("Value: {xff:?}")))
        }
    }
}

impl XffSerde for QueueItem {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            QueueItem::Event(e) => obj.insert("event", e.to_xff()),
            QueueItem::Tick(t) => obj.insert("tick", t.to_xff()),
            QueueItem::Modifier(m) => obj.insert("modifier", m.to_xff()),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(event) = obj.get("event") {
                Ok(QueueItem::Event(QueueEvent::from_xff(event)?))
            } else if let Some(tick) = obj.get("tick") {
                Ok(QueueItem::Tick(Tick::from_xff(tick)?))
            } else if let Some(modifier) = obj.get("modifier") {
                Ok(QueueItem::Modifier(QueueModifier::from_xff(modifier)?))
            } else {
                Err(new_xff_serde_error("Unable to load QueueItem from XFF")
                    .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(
                new_xff_serde_error("Unable to load QueueItem from XFF, not an object")
                    .add_ctx(format!("Value: {xff:?}")),
            )
        }
    }
}

impl QueueItem {
    /// Get the time of the item
    pub fn get_time(&self) -> Utc {
        match self {
            QueueItem::Event(e) => e.time,
            QueueItem::Tick(t) => t.get_time(),
            QueueItem::Modifier(m) => m.time,
        }
    }
    /// Check if the item is an event
    pub fn is_event(&self) -> bool {
        matches!(self, QueueItem::Event(_))
    }
    /// Check if the item is a tick
    pub fn is_tick(&self) -> bool {
        matches!(self, QueueItem::Tick(_))
    }
    /// Get the event if the item is an event
    pub fn as_event(&self) -> Option<&QueueEvent> {
        match self {
            QueueItem::Event(e) => Some(e),
            _ => None,
        }
    }
    /// Get the tick if the item is a tick
    pub fn as_tick(&self) -> Option<&Tick> {
        match self {
            QueueItem::Tick(t) => Some(t),
            _ => None,
        }
    }
}
