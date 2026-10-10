use std::time::Duration;

use aequa::{Object, XffValue, xff};
use horae::Utc;
pub struct Tick {
    pub time: Utc,
    pub kind: TickKind,
}

pub const DAY_DURATION: Duration = Duration::from_secs(60 * 60 * 24);
pub const WEEK_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 7);
pub const FORTNIGHT_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 14);

impl Tick {
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xffvalue());
        obj.insert("kind", Into::<u16>::into(self.kind));
        xff!(obj)
    }
    pub fn from_xff(value: XffValue) -> Result<Tick, ()> {
        Tick::try_from(&value)
    }
    pub fn new_from_kind(now: Utc, kind: TickKind) -> Tick {
        let time = {
            match kind {
                TickKind::Day => now + DAY_DURATION,
                TickKind::Week => now + WEEK_DURATION,
                TickKind::Fortnight => now + FORTNIGHT_DURATION,
                TickKind::Month => {
                    let now_date = now.date();
                    let (year, month) = if now_date.month == 12 {
                        (now_date.year.saturating_add(1), 1)
                    } else {
                        (now_date.year, now_date.month.saturating_add(1))
                    };
                    Utc::from_ymd_hms(year, month, 1, 0, 0, 0)
                }
                _ => {
                    let year = match kind {
                        TickKind::Year => 1,
                        TickKind::Biennial => 2,
                        TickKind::Triennial => 3,
                        TickKind::Quintennial => 5,
                        TickKind::Decennial => 10,
                        _ => panic!("Invalid tick kind"),
                    };
                    Utc::from_ymd_hms(now.date().year.saturating_add(year), 1, 1, 0, 0, 0)
                }
            }
        };
        Tick { time, kind }
    }
}

impl TryFrom<&XffValue> for Tick {
    type Error = ();
    fn try_from(value: &XffValue) -> Result<Tick, Self::Error> {
        if let Some(obj) = value.as_object()
            && let Some(time) = obj.get("time")
            && let Some(kind) = obj.get("kind")
        {
            if let Some(time) = Utc::from_xffvalue(time.clone())
                && let Some(kind) = kind.as_number()
                && let Some(kind) = kind.into_usize()
            {
                Ok(Tick {
                    time,
                    kind: (kind as u16).into(),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

// NOTE: Limited by `EventQueue`'s bit-flag to 16 at most
pub const TICK_KIND_AMOUNT: u16 = 9;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TickKind {
    Day = 0,
    Week = 1,
    Fortnight = 2,
    Month = 3,
    Year = 4,
    Biennial = 5,
    Triennial = 6,
    Quintennial = 7,
    Decennial = 8,
}

impl From<u16> for TickKind {
    fn from(value: u16) -> Self {
        match value {
            0 => TickKind::Day,
            1 => TickKind::Week,
            2 => TickKind::Fortnight,
            3 => TickKind::Month,
            4 => TickKind::Year,
            5 => TickKind::Biennial,
            6 => TickKind::Triennial,
            7 => TickKind::Quintennial,
            8 => TickKind::Decennial,
            _ => panic!("Invalid tick kind"),
        }
    }
}

impl Into<u16> for TickKind {
    fn into(self) -> u16 {
        self as u16
    }
}

#[test]
fn from_into_tick_kind() {
    for i in 0..TICK_KIND_AMOUNT {
        let tick_kind = TickKind::from(i);
        assert_eq!(Into::<u16>::into(tick_kind), i);
    }
}
