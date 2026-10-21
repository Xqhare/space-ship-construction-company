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
                TickKind::BiMonth => next_multi_month(&now, 10, 2),
                TickKind::QuarterYear => next_multi_month(&now, 9, 3),
                TickKind::HalfYear => next_multi_month(&now, 6, 6),
                _ => {
                    let year = match kind {
                        TickKind::Year => 1,
                        TickKind::Biennial => 2,
                        TickKind::Triennial => 3,
                        TickKind::Quintennial => 5,
                        TickKind::Decennial => 10,
                        TickKind::Quadranscentennial => 25,
                        TickKind::Semicentennial => 50,
                        TickKind::Centennial => 100,
                        TickKind::Sesquicentennial => 150,
                        TickKind::Bicentennial => 200,
                        _ => panic!("Invalid tick kind"),
                    };
                    Utc::from_ymd_hms(now.date().year.saturating_add(year), 1, 1, 0, 0, 0)
                }
            }
        };
        Tick { time, kind }
    }
}

fn next_multi_month(now: &Utc, month: u8, next_month: u8) -> Utc {
    let now_date = now.date();
    let (year, month) = if now_date.month > month {
        (
            now_date.year.saturating_add(1),
            now_date.month.saturating_sub(month),
        )
    } else {
        (now_date.year, now_date.month.saturating_add(next_month))
    };
    Utc::from_ymd_hms(year, month, 1, 0, 0, 0)
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

// NOTE: `TickKind` is limited by `EventQueue`'s bit-flag to 16 entries at most
pub const TICK_KIND_AMOUNT: u16 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum TickKind {
    // NOTE: Day through Year are the most common ticks
    // They are the only ticks where the simulation should be triggered to run
    Day = 0,
    Week = 1,
    Fortnight = 2,
    Month = 3,
    BiMonth = 4,
    QuarterYear = 5,
    HalfYear = 6,
    Year = 7,
    // NOTE: From here on, I think of the ticks as special triggers for events mostly.
    // Simulation-wise, they are not that important as they are always paired with a yearly tick (and day, month, bimonth, quarter year and half year)
    /// Every 2 years
    Biennial = 8,
    /// Every 3 years
    Triennial = 9,
    /// Every 5 years
    Quintennial = 10,
    /// Every 10 years
    Decennial = 11,
    /// Every 25 years
    Quadranscentennial = 12,
    // NOTE: From here on, I expect at most one or two ticks of these kinds to happen in a long to very long game.
    // They are likely to trigger special events for dedicated players
    /// Every 50 years
    Semicentennial = 13,
    /// Every 100 years
    Centennial = 14,
    // NOTE: I expect these last two kinds to almost never trigger in a save. They should be a reward for the most dedicated players
    /// Every 150 years
    Sesquicentennial = 15,
    /// Every 200 years
    Bicentennial = 16,
}

impl TickKind {
    /// Returns true if the tick is a simulation tick
    ///
    /// # Note
    /// Simulation ticks are the ticks that should trigger the simulation to run
    ///
    /// They may be day, week, fortnight, month, bimonth, quarter year or half year ticks
    pub fn is_simulation_tick(&self) -> bool {
        match self {
            TickKind::Day
            | TickKind::Week
            | TickKind::Fortnight
            | TickKind::Month
            | TickKind::BiMonth
            | TickKind::QuarterYear
            | TickKind::HalfYear
            | TickKind::Year => true,
            _ => false,
        }
    }
    /// Returns true if the tick is a special tick
    ///
    /// # Note
    /// Special ticks are the ticks that should trigger special events
    /// The Simulation should not run on these ticks
    ///
    /// They may be biennial, triennial, quintennial, decennial, quadranscentennial, semicentennial, centennial, sesquicentennial or bicentennial ticks
    pub fn is_special_tick(&self) -> bool {
        match self {
            TickKind::Biennial
            | TickKind::Triennial
            | TickKind::Quintennial
            | TickKind::Decennial
            | TickKind::Quadranscentennial
            | TickKind::Semicentennial
            | TickKind::Centennial
            | TickKind::Sesquicentennial
            | TickKind::Bicentennial => true,
            _ => false,
        }
    }
}

impl From<u16> for TickKind {
    fn from(value: u16) -> Self {
        match value {
            0 => TickKind::Day,
            1 => TickKind::Week,
            2 => TickKind::Fortnight,
            3 => TickKind::Month,
            4 => TickKind::BiMonth,
            5 => TickKind::QuarterYear,
            6 => TickKind::HalfYear,
            7 => TickKind::Year,
            8 => TickKind::Biennial,
            9 => TickKind::Triennial,
            10 => TickKind::Quintennial,
            11 => TickKind::Decennial,
            12 => TickKind::Quadranscentennial,
            13 => TickKind::Semicentennial,
            14 => TickKind::Centennial,
            15 => TickKind::Sesquicentennial,
            16 => TickKind::Bicentennial,
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

#[test]
fn bullshit_tick_kind() {
    for i in 0..TICK_KIND_AMOUNT {
        let tick_kind = TickKind::from(i);
        assert!(
            !(tick_kind.is_simulation_tick() && tick_kind.is_special_tick()),
            "Invalid tick kind. {i} is both a simulation and special tick"
        );
    }
}
