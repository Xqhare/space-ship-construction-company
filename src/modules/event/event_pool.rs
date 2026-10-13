use aequa::{Array, Object, XffValue, xff};

use crate::modules::event::EventId;

pub struct EventPools {
    background_pools: EventPool,
    story_pools: EventPool,
    special_pools: EventPool,
    people_pools: EventPool,
    production_pools: EventPool,
    science_pools: EventPool,
    ship_class_pools: EventPool,
}

impl EventPools {
    pub fn new() -> EventPools {
        EventPools {
            background_pools: EventPool::new(),
            story_pools: EventPool::new(),
            special_pools: EventPool::new(),
            people_pools: EventPool::new(),
            production_pools: EventPool::new(),
            science_pools: EventPool::new(),
            ship_class_pools: EventPool::new(),
        }
    }
}

pub struct EventPool {
    /// All events part of the pool
    pool: Vec<EventId>,
    /// All positive events part of the pool
    positive_pool: Vec<EventId>,
    /// All negative events part of the pool
    negative_pool: Vec<EventId>,
    /// All neutral events part of the pool
    neutral_pool: Vec<EventId>,
}

impl EventPool {
    pub fn new() -> EventPool {
        EventPool {
            pool: Vec::new(),
            positive_pool: Vec::new(),
            negative_pool: Vec::new(),
            neutral_pool: Vec::new(),
        }
    }
    pub fn from_xff(value: &XffValue) -> Result<EventPool, ()> {
        value.try_into()
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("pool", pool_to_xff(&self.pool));
        obj.insert("positive_pool", pool_to_xff(&self.positive_pool));
        obj.insert("negative_pool", pool_to_xff(&self.negative_pool));
        obj.insert("neutral_pool", pool_to_xff(&self.neutral_pool));
        xff!(obj)
    }
}

fn xff_to_pool(ary: &Array) -> Vec<EventId> {
    let mut out_pool: Vec<EventId> = Vec::new();
    for event_id in ary {
        if let Some(event_id) = event_id.as_number()
            && let Some(event_id) = event_id.into_usize()
        {
            out_pool.push(event_id as u32);
        }
    }
    out_pool
}

fn pool_to_xff(pool: &Vec<EventId>) -> XffValue {
    let mut ary = Array::new();
    for event_id in pool {
        ary.push(*event_id);
    }
    xff!(ary)
}

impl TryInto<EventPool> for &XffValue {
    type Error = ();
    fn try_into(self) -> Result<EventPool, Self::Error> {
        if let Some(obj) = self.as_object()
            && let Some(pool) = obj.get("pool")
            && let Some(positive_pool) = obj.get("positive_pool")
            && let Some(negative_pool) = obj.get("negative_pool")
            && let Some(neutral_pool) = obj.get("neutral_pool")
        {
            if let Some(pool) = pool.as_array()
                && let Some(positive_pool) = positive_pool.as_array()
                && let Some(negative_pool) = negative_pool.as_array()
                && let Some(neutral_pool) = neutral_pool.as_array()
            {
                let out_pool = xff_to_pool(pool);
                let positive_out_pool = xff_to_pool(positive_pool);
                let negative_out_pool = xff_to_pool(negative_pool);
                let neutral_out_pool = xff_to_pool(neutral_pool);
                Ok(EventPool {
                    pool: out_pool,
                    positive_pool: positive_out_pool,
                    negative_pool: negative_out_pool,
                    neutral_pool: neutral_out_pool,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
