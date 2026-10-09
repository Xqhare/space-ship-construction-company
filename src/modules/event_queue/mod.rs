use aequa::{Array, Duration, XffValue, xff};
use horae::Utc;

use crate::modules::event::Event;

#[non_exhaustive]
pub struct EventQueue {
    /// Sorted by time
    events: Vec<Event>,
}

impl EventQueue {
    /// Create a new, empty, event queue
    ///
    /// To create an event queue with events, use [`XffValue::try_into`]
    pub fn new() -> EventQueue {
        EventQueue { events: Vec::new() }
    }
    /// Add an event to the queue
    ///
    /// # NOTE
    /// This inserts the event into the correct position in the queue based on the time.
    pub fn add_event(&mut self, event: Event) {
        let mut idx = 0;
        self.events
            .iter()
            .position(|e| e.time > event.time)
            .map(|i| idx = i);
        self.events.insert(idx, event);
    }
    /// Push an event to the end of the queue
    ///
    /// # NOTE
    /// This is not sorted. Only use if the order of push calls does not matter or is already sorted
    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
    /// Get the next event to fire and the duration until it fires
    pub fn get_next_event(&mut self, now: Utc, duration: Duration) -> Option<(&Event, Duration)> {
        let end = now + std::time::Duration::from_millis(duration.as_millis());
        if let Some(event) = self.events.iter().find(|e| e.time < end) {
            let dur = end - event.time;
            Some((event, Duration::from_millis(dur.as_millis() as u64)))
        } else {
            None
        }
    }
}

impl Into<XffValue> for &EventQueue {
    fn into(self) -> XffValue {
        let mut ary = Array::new();
        for event in &self.events {
            ary.push(event.to_xff());
        }
        xff!(ary)
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
        let mut queue = EventQueue::new();
        for event in ary.iter() {
            queue.push_event(event.try_into()?);
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
