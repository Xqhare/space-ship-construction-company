use aequa::XffValue;
use athena::float::HpFloat;
use horae::Utc;

use crate::modules::modifier::Modifier;

pub mod event_id_pool;
pub mod event_queue;
pub mod event_store;

pub type EventId = u32;
pub type EventChoiceId = u8;
pub type BaseValueName = String;
pub type FlagName = String;

pub struct Event {
    pub id: EventId,
    pub title: String,
    pub description: String,
    pub is_triggered_only: bool,
    pub is_player_only: bool,
    pub triggers: Vec<EventTrigger>,
    pub immediate_effects: Vec<Modifier>,
    pub actions: Vec<EventChoice>,
}

impl Event {
    pub fn to_xff(&self) -> XffValue {
        todo!()
    }
    pub fn from_xff(value: &XffValue) -> Result<Event, ()> {
        todo!()
    }
}

pub struct EventChoice {
    pub id: EventChoiceId,
    pub text: String,
    pub ai_chance: HpFloat,
    pub rewards: Vec<ChoiceReward>,
}

pub enum ChoiceReward {}

pub enum EventTrigger {
    Date(EventOp, Utc),
    PlayerStat(BaseValueName, EventOp, HpFloat),
    PlayerFlag(FlagName, bool),
    WorldStat(BaseValueName, EventOp, HpFloat),
    Event(EventId, Option<EventChoiceId>),
}

pub enum EventOp {
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    NotEqual,
    Not,
}
