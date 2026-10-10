use std::time::Duration;

use aequa::{Array, Object, XffValue, xff};
use athena::bit_flags::BitFlag;
use horae::Utc;

pub mod queue_event;

use crate::modules::{
    event_queue::queue_event::QueueEvent,
    tick::{TICK_KIND_AMOUNT, Tick, TickKind},
};

#[non_exhaustive]
pub struct EventQueue {
    /// Sorted by time; Contains events and ticks
    events: Vec<QueueItem>,
    /// Tracks which ticks are present in the queue
    present_ticks: BitFlag<u16>,
}

/// Helper enum for the event queue
///
/// Contains either an event or a tick
pub enum QueueItem {
    /// An event for the user / player to interact with
    /// Some need to interrupt the passing of time, others don't
    Event(QueueEvent),
    /// A tick (day, week, month, year, etc) for internal use
    Tick(Tick),
}

impl QueueItem {
    /// Get the time of the item
    pub fn get_time(&self) -> Utc {
        match self {
            QueueItem::Event(e) => e.time,
            QueueItem::Tick(t) => t.time,
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

impl EventQueue {
    /// Create a new, empty, event queue
    ///
    /// To create an event queue with events, use [`XffValue::try_into`]
    pub fn new(now: Utc) -> EventQueue {
        let mut out = EventQueue {
            events: Vec::new(),
            present_ticks: BitFlag::new(),
        };
        while let Some(missing_tick) = out.tick_kind_missing() {
            out.new_tick(now, missing_tick);
        }
        out
    }
    /// Create a new, event queue with no events nor ticks in the queue.
    ///
    /// Use for testing and loading of save-states
    fn new_empty() -> EventQueue {
        EventQueue {
            events: Vec::new(),
            present_ticks: BitFlag::new(),
        }
    }
    /// Return the number of events in the queue
    ///
    /// The number includes all ticks and events scheduled in the future
    pub fn queue_len(&self) -> usize {
        self.events.len()
    }
    /// Check if any ticks are missing
    ///
    /// The first missing `TickKind` is returned.
    ///
    /// Use together with [`EventQueue::new_tick`]:
    /// ```
    /// use horae::Utc;
    /// use space_ship_construction_company::modules::event_queue::EventQueue;
    /// let mut queue = EventQueue::new(Utc::now());
    /// while let Some(missing_tick) = queue.tick_kind_missing() {
    ///     queue.new_tick(Utc::now(), missing_tick);
    /// }
    /// assert!(queue.tick_kind_missing().is_none());
    /// ```
    pub fn tick_kind_missing(&self) -> Option<TickKind> {
        for n in 0..TICK_KIND_AMOUNT {
            if !self.present_ticks.read(n as usize) {
                return Some(TickKind::from(n));
            }
        }
        None
    }
    /// Add a new tick to the queue.
    ///
    /// If a tick of the same kind is already present, it is not added.
    pub fn new_tick(&mut self, now: Utc, kind: TickKind) {
        if !self.present_ticks.read(kind as usize) {
            let tick = Tick::new_from_kind(now, kind);
            let mut idx = 0;
            self.events
                .iter()
                .position(|e| e.get_time() > tick.time)
                .map(|i| idx = i);
            self.present_ticks.set(kind as usize);
            self.events.insert(idx, QueueItem::Tick(tick));
        }
    }
    /// Add an event to the queue
    ///
    /// # NOTE
    /// This inserts the event into the correct position in the queue based on the time.
    pub fn add_event(&mut self, event: QueueEvent) {
        let mut idx = 0;
        self.events
            .iter()
            .position(|e| e.get_time() > event.time)
            .map(|i| idx = i);
        self.events.insert(idx, QueueItem::Event(event));
    }
    /// Push an event to the end of the queue
    ///
    /// # NOTE
    /// This is not sorted. Only use if the order of push calls does not matter or is already sorted
    fn push_event(&mut self, event: QueueEvent) {
        self.events.push(QueueItem::Event(event));
    }
    /// Push a tick to the end of the queue
    ///
    /// # NOTE
    /// This is not sorted. Only use if the order of push calls does not matter or is already sorted.
    ///
    /// Further, this should only be called when loading in a saved game
    fn push_tick(&mut self, tick: Tick) {
        self.events.push(QueueItem::Tick(tick));
    }
    /// Get the next event to fire and the duration until it fires
    ///
    /// # Returns
    /// A `QueueItem` and the duration until it fires
    ///
    /// The `QueueItem` is either an event or a tick
    pub fn get_next_event(
        &mut self,
        now: Utc,
        duration: Duration,
    ) -> Option<(QueueItem, Duration)> {
        let end = now + duration;
        if self.events.is_empty() {
            return None;
        } else {
            if self.events[0].get_time() > end {
                return None;
            } else {
                if let Some(event_idx) = self.events.iter().position(|e| e.get_time() < end) {
                    let fin_event = self.events.remove(event_idx);
                    if let Some(tick_event) = fin_event.as_tick() {
                        self.present_ticks.clear(tick_event.kind as usize);
                        self.new_tick(now, tick_event.kind);
                    }

                    let dur = end - fin_event.get_time();
                    Some((fin_event, dur))
                } else {
                    None
                }
            }
        }
    }
}

impl Into<XffValue> for &EventQueue {
    fn into(self) -> XffValue {
        let mut obj = Object::new();
        let mut event_ary = Array::new();
        for event in &self.events {
            event_ary.push(event.to_xff());
        }
        obj.insert("events", event_ary);
        let mut tick_ary = Array::new();
        for n in 0..16 {
            let bool = self.present_ticks.read(n);
            tick_ary.push(bool);
        }
        obj.insert("ticks", tick_ary);
        xff!(obj)
    }
}

impl Into<XffValue> for EventQueue {
    fn into(self) -> XffValue {
        (&self).into()
    }
}

impl TryFrom<&XffValue> for EventQueue {
    type Error = ();
    fn try_from(value: &XffValue) -> Result<EventQueue, Self::Error> {
        let ary = value.as_array().ok_or(())?;
        let mut queue = EventQueue::new_empty();
        for event in ary.iter() {
            let event_obj = event.as_object().ok_or(())?;
            if event_obj.contains_key("event_id") {
                let event = QueueEvent::from_xff(event.clone())?;
                queue.push_event(event);
            } else {
                let tick = Tick::from_xff(event.clone())?;
                queue.push_tick(tick);
            }
        }
        Ok(queue)
    }
}

impl TryFrom<XffValue> for EventQueue {
    type Error = ();
    fn try_from(value: XffValue) -> Result<EventQueue, Self::Error> {
        (&value).try_into()
    }
}
