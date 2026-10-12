use std::{collections::VecDeque, time::Duration};

use aequa::{Array, Object, XffValue, xff};
use athena::bit_flags::BitFlag;
use horae::Utc;

pub mod queue_event;
pub mod queue_item;
pub mod tick;

use crate::modules::event::event_queue::{
    queue_event::QueueEvent,
    queue_item::QueueItem,
    tick::{TICK_KIND_AMOUNT, Tick, TickKind},
};

#[non_exhaustive]
pub struct EventQueue {
    /// Sorted by time; Contains events and ticks
    events: VecDeque<QueueItem>,
    /// Tracks which ticks are present in the queue
    present_ticks: BitFlag<u16>,
}
impl EventQueue {
    /// Create a new, empty, event queue
    ///
    /// To create an event queue with events, use [`XffValue::try_into`]
    pub fn new(now: Utc) -> EventQueue {
        let mut out = EventQueue {
            events: VecDeque::new(),
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
            events: VecDeque::new(),
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
            let idx = match self
                .events
                .binary_search_by_key(&tick.time, |e| e.get_time())
            {
                Ok(i) => i,
                Err(i) => i,
            };
            self.present_ticks.set(kind as usize);
            self.events.insert(idx, QueueItem::Tick(tick));
        }
    }
    /// Add an event to the queue
    ///
    /// # NOTE
    /// This inserts the event into the correct position in the queue based on the time.
    pub fn add_event(&mut self, event: QueueEvent) {
        let idx = match self
            .events
            .binary_search_by_key(&event.time, |e| e.get_time())
        {
            Ok(i) => i,
            Err(i) => i,
        };
        self.events.insert(idx, QueueItem::Event(event));
    }
    /// Push an event to the end of the queue
    ///
    /// # NOTE
    /// This is not sorted. Only use if the order of push calls does not matter or is already sorted
    ///
    /// Further, this should only be called when loading in a saved game
    fn push_event(&mut self, event: QueueEvent) {
        self.events.push_back(QueueItem::Event(event));
    }
    /// Push a tick to the end of the queue
    ///
    /// # NOTE
    /// This is not sorted. Only use if the order of push calls does not matter or is already sorted.
    ///
    /// Further, this should only be called when loading in a saved game
    fn push_tick(&mut self, tick: Tick) {
        self.events.push_back(QueueItem::Tick(tick));
    }
    /// Get the next event to fire and the duration until it fires
    ///
    /// # Returns
    /// A `QueueItem` and the duration until it fires
    ///
    /// The `QueueItem` is either an event or a tick.
    ///
    /// The `Duration` is the time left over after reaching this event.
    ///
    /// Should the very next event be in 24 hours for example:
    /// If the call is made with a duration of 24 hours, the duration returned will be 0 hours.
    /// If the call is made with a duration of 25 hours, the duration returned will be 1 hour.
    pub fn get_next_event(
        &mut self,
        now: Utc,
        duration: Duration,
    ) -> Option<(QueueItem, Duration)> {
        let end = now + duration;

        if let Some(front) = self.events.front()
            && front.get_time() <= end
        {
            let event = self.events.pop_front().expect("Front must exist");
            if let Some(tick_event) = event.as_tick() {
                self.present_ticks.clear(tick_event.kind as usize);
                self.new_tick(now, tick_event.kind);
            }
            let final_duration = end - event.get_time();
            return Some((event, final_duration));
        }
        None
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

#[test]
fn queue_contains_all_ticks() {
    let now = Utc::now();
    let queue = EventQueue::new(now);
    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
}

#[test]
fn queue_only_returns_next_item() {
    let now = Utc::from_ymd_hms(2000, 1, 1, 0, 0, 0);
    let mut queue = EventQueue::new(now);
    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
    let next_event = queue.get_next_event(now, Duration::from_hours(124));
    assert!(next_event.is_some());
    let next_event = next_event.unwrap();
    assert!(next_event.0.is_tick());
    println!("{:?}", next_event.1);
    assert!(next_event.1 == Duration::from_hours(100));
    let next_tick = next_event.0.as_tick().unwrap();
    assert!(next_tick.kind == TickKind::Day);
    assert!(next_tick.time == now + Duration::from_hours(24));

    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
}

#[test]
fn queue_regenerates_ticks() {
    let now = Utc::from_ymd_hms(2000, 1, 1, 0, 0, 0);
    let mut queue = EventQueue::new(now);
    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
    let next_event = queue.get_next_event(now, Duration::from_hours(24));
    assert!(next_event.is_some());
    let next_event = next_event.unwrap();
    assert!(next_event.0.is_tick());
    println!("{:?}", next_event.1);
    assert!(next_event.1 == Duration::from_hours(0));
    let next_tick = next_event.0.as_tick().unwrap();
    assert!(next_tick.kind == TickKind::Day);

    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
}
