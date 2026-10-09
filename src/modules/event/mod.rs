use aequa::{Object, XffValue, xff};
use horae::Utc;

#[derive(Debug, Clone)]
pub struct Event {
    /// UNIX Timestamp of when the event will fire
    pub time: Utc,
    /// ID of the event
    pub id: usize,
    pub user_interupt: bool,
}

impl Event {
    pub fn new(time: Utc, id: usize) -> Event {
        Event { time, id }
    }
    pub fn from_xff(value: XffValue) -> Result<Event, ()> {
        value.try_into()
    }
    pub fn to_xff(&self) -> XffValue {
        self.into()
    }
}

impl Into<XffValue> for &Event {
    fn into(self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xffvalue());
        obj.insert("id", self.id);
        xff!(obj)
    }
}

impl Into<XffValue> for Event {
    fn into(self) -> XffValue {
        (&self).into()
    }
}

impl TryFrom<&XffValue> for Event {
    type Error = ();
    fn try_from(value: &XffValue) -> Result<Event, Self::Error> {
        if let Some(obj) = value.as_object()
            && let Some(time) = obj.get("time")
            && let Some(id) = obj.get("id")
        {
            if let Some(id) = id.as_number()
                && let Some(id) = id.into_usize()
                && let Some(time) = Utc::from_xffvalue(time.clone())
            {
                Ok(Event { time, id })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryFrom<XffValue> for Event {
    type Error = ();
    fn try_from(value: XffValue) -> Result<Event, Self::Error> {
        (&value).try_into()
    }
}
