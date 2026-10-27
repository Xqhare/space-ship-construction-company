use std::{collections::VecDeque, time::Duration};

use aequa::{Array, Object, XffSerde, XffValue, xff};
use athena::bit_flags::BitFlag;
use horae::Utc;

pub mod queue_event;
pub mod queue_item;
pub mod tick;

use crate::{
    error::new_xff_serde_error,
    modules::event::event_queue::{
        queue_item::QueueItem,
        tick::{
            Tick, TickKind,
            simulation::{SIMULATION_TICK_KIND_AMOUNT, SimulationTick, SimulationTickKind},
            special::{SPECIAL_TICK_KIND_AMOUNT, SpecialTick, SpecialTickKind},
        },
    },
};

#[non_exhaustive]
pub struct EventQueue {
    /// Sorted by time; Contains events and ticks.
    /// Items are grouped together if they share the exact same timestamp
    events: VecDeque<Vec<QueueItem>>,
    /// Tracks which ticks are present in the queue
    present_simulation_ticks: BitFlag<u8>,
    present_special_ticks: BitFlag<u16>,
}

impl XffSerde for EventQueue {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        let events = {
            let mut arr = Array::new();
            for event_group in &self.events {
                let mut event_group_arr = Array::new();
                for event in event_group {
                    event_group_arr.push(event.to_xff());
                }
                arr.push(event_group_arr);
            }
            arr
        };
        obj.insert("events", events);
        let present_simulation_ticks = {
            let mut arr = Array::new();
            for i in 0..SIMULATION_TICK_KIND_AMOUNT {
                arr.push(self.present_simulation_ticks.read(i as usize));
            }
            arr
        };
        obj.insert("present_simulation_ticks", present_simulation_ticks);
        let present_special_ticks = {
            let mut arr = Array::new();
            for i in 0..SPECIAL_TICK_KIND_AMOUNT {
                arr.push(self.present_special_ticks.read(i as usize));
            }
            arr
        };
        obj.insert("present_special_ticks", present_special_ticks);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(events) = obj.get("events")
            && let Some(events) = events.as_array()
            && let Some(present_simulation_ticks) = obj.get("present_simulation_ticks")
            && let Some(present_simulation_ticks) = present_simulation_ticks.as_array()
            && let Some(present_special_ticks) = obj.get("present_special_ticks")
            && let Some(present_special_ticks) = present_special_ticks.as_array()
        {
            let events: VecDeque<Vec<QueueItem>> = {
                let mut out = VecDeque::new();
                for event_group in events {
                    if let Some(event_group) = event_group.as_array() {
                        let mut event_group_out = Vec::new();
                        for event in event_group {
                            event_group_out.push(QueueItem::from_xff(event)?);
                        }
                        out.push_back(event_group_out);
                    }
                }
                out
            };
            let present_simulation_ticks = {
                let mut out = BitFlag::new();
                for (i, elem) in present_simulation_ticks.iter().enumerate() {
                    if let Some(elem) = elem.as_boolean()
                        && *elem
                    {
                        out.set(i);
                    }
                }
                out
            };
            let present_special_ticks = {
                let mut out = BitFlag::new();
                for (i, elem) in present_special_ticks.iter().enumerate() {
                    if let Some(elem) = elem.as_boolean()
                        && *elem
                    {
                        out.set(i);
                    }
                }
                out
            };
            Ok(EventQueue {
                events,
                present_simulation_ticks: BitFlag::from(present_simulation_ticks),
                present_special_ticks: BitFlag::from(present_special_ticks),
            })
        } else {
            Err(new_xff_serde_error("Could not load EventQueue from XFF")
                .add_ctx(format!("Value: {xff:?}")))
        }
    }
}

impl EventQueue {
    /// Create a new, empty, event queue
    pub fn new(now: Utc) -> EventQueue {
        let mut out = Self::new_empty();
        while let Some(missing_tick) = out.tick_kind_missing() {
            out.new_tick(now, missing_tick);
        }
        out
    }
    /// Create a new, event queue with no events nor ticks in the queue.
    fn new_empty() -> EventQueue {
        EventQueue {
            // The real min would be 16 + 8 for the ticks
            // There will be many more events added to this queue
            events: VecDeque::with_capacity(128),
            present_simulation_ticks: BitFlag::new(),
            present_special_ticks: BitFlag::new(),
        }
    }
    /// Return the number of events in the queue
    ///
    /// The number includes all ticks and events scheduled in the future
    pub fn queue_len(&self) -> usize {
        let mut len = 0;
        for event in &self.events {
            len += event.len();
        }
        len
    }
    /// Check if any ticks are missing.
    /// Checks both simulation and special ticks.
    ///
    /// The first missing `TickKind` is returned.
    ///
    /// Use together with [`EventQueue::new_tick`]:
    /// ```
    /// use horae::Utc;
    /// use space_ship_construction_company::modules::event::event_queue::EventQueue;
    /// let mut queue = EventQueue::new(Utc::now());
    /// while let Some(missing_tick) = queue.tick_kind_missing() {
    ///     queue.new_tick(Utc::now(), missing_tick);
    /// }
    /// assert!(queue.tick_kind_missing().is_none());
    /// ```
    fn tick_kind_missing(&self) -> Option<TickKind> {
        for n in 0..SIMULATION_TICK_KIND_AMOUNT {
            if !self.present_simulation_ticks.read(n as usize) {
                return Some(TickKind::Simulation(SimulationTickKind::from(n)));
            }
        }
        for n in 0..SPECIAL_TICK_KIND_AMOUNT {
            if !self.present_special_ticks.read(n as usize) {
                return Some(TickKind::Special(SpecialTickKind::from(n)));
            }
        }
        None
    }
    /// Add a new tick to the queue.
    ///
    /// If a tick of the same kind is already present, it is not added.
    fn new_tick(&mut self, now: Utc, kind: TickKind) {
        let item = match kind {
            TickKind::Simulation(sim) => {
                if !self.present_simulation_ticks.read(sim as usize) {
                    self.present_simulation_ticks.set(sim as usize);
                    QueueItem::Tick(Tick::Simulation(SimulationTick::new_from_kind(now, sim)))
                } else {
                    return;
                }
            }
            TickKind::Special(spec) => {
                if !self.present_special_ticks.read(spec as usize) {
                    self.present_special_ticks.set(spec as usize);
                    QueueItem::Tick(Tick::Special(SpecialTick::new_from_kind(now, spec)))
                } else {
                    return;
                }
            }
        };
        self.add_item(item);
    }
    /// Add an event to the queue
    ///
    /// # NOTE
    /// This inserts the event into the correct position in the queue based on the time.
    pub fn add_item(&mut self, item: QueueItem) {
        for (idx, group) in self.events.iter_mut().enumerate() {
            if let Some(first) = group.first() {
                let group_time = first.get_time();
                let item_time = item.get_time();
                if group_time == item_time {
                    group.push(item);
                    return;
                } else if group_time > item_time {
                    self.events.insert(idx, vec![item]);
                    return;
                }
            }
        }
        self.events.push_back(vec![item]);
    }
    /// Get the next event to fire and the duration until it fires
    ///
    /// # Returns
    /// A `Vec` of `QueueItem` and the duration until it fires
    ///
    /// The elements of the `Vec` all have the same timestamp.
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
    ) -> Option<(Vec<QueueItem>, Duration)> {
        let end = now + duration;

        if let Some(front) = self.events.front()
            && let Some(front_first) = front.first()
            && front_first.get_time() <= end
        {
            let final_duration = end - front_first.get_time();
            let events = self.events.pop_front().expect("Front must exist");
            for event in &events {
                if let Some(tick_event) = event.as_tick() {
                    match tick_event {
                        Tick::Simulation(sim) => {
                            self.present_simulation_ticks.clear(sim.kind as usize);
                            self.new_tick(now, TickKind::Simulation(sim.kind));
                        }
                        Tick::Special(spec) => {
                            self.present_special_ticks.clear(spec.kind as usize);
                            self.new_tick(now, TickKind::Special(spec.kind));
                        }
                    }
                }
            }
            return Some((events, final_duration));
        }
        None
    }
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
    assert!(!next_event.0.is_empty());
    let (next_event, dur) = (next_event.0.first().unwrap(), next_event.1);
    assert!(next_event.is_tick());
    assert!(dur == Duration::from_hours(100));
    let next_tick = next_event.as_tick().unwrap();
    let (kind, time) = match next_tick {
        Tick::Simulation(sim) => (TickKind::Simulation(sim.kind), sim.time),
        Tick::Special(spec) => (TickKind::Special(spec.kind), spec.time),
    };
    assert!(kind == TickKind::Simulation(SimulationTickKind::Day));
    assert!(time == now + Duration::from_hours(24));

    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16 + 8);
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
    assert!(!next_event.0.is_empty());
    let (next_event, dur) = (next_event.0.first().unwrap(), next_event.1);
    assert!(next_event.is_tick());
    assert!(dur == Duration::from_hours(0));
    let next_tick = next_event.as_tick().unwrap();
    let kind = match next_tick {
        Tick::Simulation(sim) => TickKind::Simulation(sim.kind),
        Tick::Special(spec) => TickKind::Special(spec.kind),
    };
    assert!(kind == TickKind::Simulation(SimulationTickKind::Day));

    assert!(queue.tick_kind_missing().is_none());
    assert!(queue.queue_len() == 16);
}
