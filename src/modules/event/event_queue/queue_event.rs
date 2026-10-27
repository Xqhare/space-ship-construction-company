use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;

use crate::error::new_xff_serde_error;

#[derive(Debug, Clone)]
pub struct QueueEvent {
    /// UNIX Timestamp of when the event will fire
    pub time: Utc,
    /// ID of the event
    pub event_id: usize,
    /// If the event needs to interrupt the event queue
    pub user_interupt: bool,
}

impl QueueEvent {
    pub fn new(time: Utc, event_id: usize, user_interupt: bool) -> QueueEvent {
        QueueEvent {
            time,
            event_id,
            user_interupt,
        }
    }
}

impl XffSerde for QueueEvent {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xff());
        obj.insert("event_id", self.event_id);
        obj.insert("user_interupt", self.user_interupt);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(time) = obj.get("time")
            && let Some(id) = obj.get("event_id")
            && let Some(user_interupt) = obj.get("user_interupt")
        {
            if let Some(id) = id.as_number()
                && let Some(id) = id.into_usize()
                && let Some(user_interupt) = user_interupt.as_boolean()
            {
                let time = Utc::from_xff(time)?;
                Ok(QueueEvent {
                    time,
                    event_id: id,
                    user_interupt: *user_interupt,
                })
            } else {
                Err(new_xff_serde_error("Unable to load QueueEvent from XFF")
                    .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(new_xff_serde_error("Unable to load QueueEvent from XFF")
                .add_ctx(format!("Value: {xff:?}")))
        }
    }
}
