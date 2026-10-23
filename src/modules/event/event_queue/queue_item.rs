use aequa::XffValue;
use horae::Utc;

use crate::modules::event::event_queue::{queue_event::QueueEvent, tick::Tick};

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
    pub stat_box_location: StatBoxLocation,
}

impl QueueItem {
    /// Get the time of the item
    pub fn get_time(&self) -> Utc {
        match self {
            QueueItem::Event(e) => e.time,
            QueueItem::Tick(t) => t.time,
            QueueItem::Modifier(m) => m.time,
        }
    }
    /// Convert the item to a `XffValue`
    pub fn to_xff(&self) -> XffValue {
        match self {
            QueueItem::Event(e) => e.to_xff(),
            QueueItem::Tick(t) => t.to_xff(),
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
