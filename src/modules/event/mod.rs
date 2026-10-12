use athena::float::HpFloat;
use horae::Utc;

pub mod event_pool;
pub mod event_queue;
pub mod event_store;

pub type EventId = u32;
pub type EventChoiceId = u8;

pub struct Event {
    pub id: EventId,
    pub title: String,
    pub description: String,
    pub is_triggered_only: bool,
    pub is_player_only: bool,
    pub triggers: Vec<EventTrigger>,
    pub immediate_effects: Vec<EffectId>,
    pub actions: Vec<EventChoice>,
}

pub struct EventChoice {
    pub id: EventChoiceId,
    pub text: String,
    pub ai_chance: HpFloat,
    pub rewards: Vec<ChoiceReward>,
}

pub enum ChoiceReward {
    Money(HpFloat),
    Modifier(Modifier),
    Event(EventId, Option<EventChoiceId>),
}

pub enum EventTrigger {
    Date(EventOp, Utc),
    Money(EventOp, HpFloat),
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
