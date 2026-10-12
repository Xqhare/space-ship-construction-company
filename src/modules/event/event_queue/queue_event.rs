use aequa::{Object, XffValue, xff};
use horae::Utc;

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
    pub fn from_xff(value: XffValue) -> Result<QueueEvent, ()> {
        value.try_into()
    }
    pub fn to_xff(&self) -> XffValue {
        self.into()
    }
}

impl Into<XffValue> for &QueueEvent {
    fn into(self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xffvalue());
        obj.insert("event_id", self.event_id);
        obj.insert("user_interupt", self.user_interupt);
        xff!(obj)
    }
}

impl Into<XffValue> for QueueEvent {
    fn into(self) -> XffValue {
        (&self).into()
    }
}

impl TryFrom<&XffValue> for QueueEvent {
    type Error = ();
    fn try_from(value: &XffValue) -> Result<QueueEvent, Self::Error> {
        if let Some(obj) = value.as_object()
            && let Some(time) = obj.get("time")
            && let Some(id) = obj.get("event_id")
            && let Some(user_interupt) = obj.get("user_interupt")
        {
            if let Some(id) = id.as_number()
                && let Some(id) = id.into_usize()
                && let Some(user_interupt) = user_interupt.as_boolean()
                && let Some(time) = Utc::from_xffvalue(time.clone())
            {
                Ok(QueueEvent {
                    time,
                    event_id: id,
                    user_interupt: *user_interupt,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryFrom<XffValue> for QueueEvent {
    type Error = ();
    fn try_from(value: XffValue) -> Result<QueueEvent, Self::Error> {
        (&value).try_into()
    }
}
