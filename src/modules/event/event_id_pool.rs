use std::path::Path;

use aequa::{Array, Object, XffValue, xff};

use crate::{
    error::{SSCCResult, new_startup_error},
    modules::event::EventId,
};

pub struct EventIdPools {
    background_pools: EventIdPool,
    story_pools: EventIdPool,
    special_pools: EventIdPool,
    people_pools: EventIdPool,
    production_pools: EventIdPool,
    science_pools: EventIdPool,
    ship_class_pools: EventIdPool,
}

impl EventIdPools {
    pub fn new() -> EventIdPools {
        EventIdPools {
            background_pools: EventIdPool::new(),
            story_pools: EventIdPool::new(),
            special_pools: EventIdPool::new(),
            people_pools: EventIdPool::new(),
            production_pools: EventIdPool::new(),
            science_pools: EventIdPool::new(),
            ship_class_pools: EventIdPool::new(),
        }
    }
    pub fn init_new<P: AsRef<Path>>(all_events_path: P) -> SSCCResult<EventIdPools> {
        Ok(EventIdPools::new())
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("background_pools", self.background_pools.to_xff());
        obj.insert("story_pools", self.story_pools.to_xff());
        obj.insert("special_pools", self.special_pools.to_xff());
        obj.insert("people_pools", self.people_pools.to_xff());
        obj.insert("production_pools", self.production_pools.to_xff());
        obj.insert("science_pools", self.science_pools.to_xff());
        obj.insert("ship_class_pools", self.ship_class_pools.to_xff());
        xff!(obj)
    }
    pub fn from_xff(value: &XffValue) -> SSCCResult<EventIdPools> {
        if let Some(obj) = value.as_object()
            && let Some(background_pools) = obj.get("background_pools")
            && let Ok(background_pools) = background_pools.try_into()
            && let Some(story_pools) = obj.get("story_pools")
            && let Ok(story_pools) = story_pools.try_into()
            && let Some(special_pools) = obj.get("special_pools")
            && let Ok(special_pools) = special_pools.try_into()
            && let Some(people_pools) = obj.get("people_pools")
            && let Ok(people_pools) = people_pools.try_into()
            && let Some(production_pools) = obj.get("production_pools")
            && let Ok(production_pools) = production_pools.try_into()
            && let Some(science_pools) = obj.get("science_pools")
            && let Ok(science_pools) = science_pools.try_into()
            && let Some(ship_class_pools) = obj.get("ship_class_pools")
            && let Ok(ship_class_pools) = ship_class_pools.try_into()
        {
            Ok(EventIdPools {
                background_pools,
                story_pools,
                special_pools,
                people_pools,
                production_pools,
                science_pools,
                ship_class_pools,
            })
        } else {
            Err(new_startup_error("Unable to load EventPools from XFF"))
        }
    }
}

pub struct EventIdPool {
    /// All events part of the pool
    pool: Vec<EventId>,
    /// All positive events part of the pool
    positive_pool: Vec<EventId>,
    /// All negative events part of the pool
    negative_pool: Vec<EventId>,
    /// All neutral events part of the pool
    neutral_pool: Vec<EventId>,
}

impl EventIdPool {
    pub fn new() -> EventIdPool {
        EventIdPool {
            pool: Vec::new(),
            positive_pool: Vec::new(),
            negative_pool: Vec::new(),
            neutral_pool: Vec::new(),
        }
    }
    pub fn from_xff(value: &XffValue) -> Result<EventIdPool, ()> {
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

impl TryInto<EventIdPool> for &XffValue {
    type Error = ();
    fn try_into(self) -> Result<EventIdPool, Self::Error> {
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
                Ok(EventIdPool {
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
