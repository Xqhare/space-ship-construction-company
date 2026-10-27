use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;
use nemesis::NemesisResultExt;

use crate::{
    consts::durations::{DAY_DUR_SECS, FORTNIGHT_DUR_SECS, WEEK_DUR_SECS},
    error::new_xff_serde_error,
};

// NOTE: `SpecialTickKind` is limited by `EventQueue`'s bit-flag to 8 entries at most
pub const SIMULATION_TICK_KIND_AMOUNT: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimulationTick {
    pub time: Utc,
    pub kind: SimulationTickKind,
}
impl XffSerde for SimulationTick {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xff());
        obj.insert("kind", Into::<u8>::into(self.kind));
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(time) = obj.get("time")
            && let Some(kind) = obj.get("kind")
            && let Some(kind) = kind.as_number()
            && let Some(kind) = kind.into_usize()
        {
            let time = Utc::from_xff(time).add_ctx("Cannot load SpecialTick from XFF")?;
            let kind = SimulationTickKind::from(kind as u8);
            Ok(SimulationTick { time, kind })
        } else {
            Err(new_xff_serde_error("Unable to load SpecialTick from XFF")
                .add_ctx(format!("Value: {xff:?}")))
        }
    }
}
impl SimulationTick {
    pub fn new_from_kind(now: Utc, simulation_tick_kind: SimulationTickKind) -> Self {
        let time = match simulation_tick_kind {
            SimulationTickKind::Day => now + DAY_DUR_SECS,
            SimulationTickKind::Week => now + WEEK_DUR_SECS,
            SimulationTickKind::Fortnight => now + FORTNIGHT_DUR_SECS,
            SimulationTickKind::Month => {
                let now_date = now.date();
                let (year, month) = if now_date.month == 12 {
                    (now_date.year.saturating_add(1), 1)
                } else {
                    (now_date.year, now_date.month.saturating_add(1))
                };
                Utc::from_ymd_hms(year, month, 1, 0, 0, 0)
            }
            SimulationTickKind::BiMonth => next_multi_month(&now, 10, 2),
            SimulationTickKind::QuarterYear => next_multi_month(&now, 9, 3),
            SimulationTickKind::HalfYear => next_multi_month(&now, 6, 6),
            SimulationTickKind::Year => {
                Utc::from_ymd_hms(now.date().year.saturating_add(1), 1, 1, 0, 0, 0)
            }
        };
        SimulationTick {
            time,
            kind: simulation_tick_kind,
        }
    }
}
/// They are the only ticks where the simulation should be triggered to run
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SimulationTickKind {
    Day = 0,
    Week = 1,
    Fortnight = 2,
    Month = 3,
    BiMonth = 4,
    QuarterYear = 5,
    HalfYear = 6,
    Year = 7,
}

impl From<u8> for SimulationTickKind {
    fn from(value: u8) -> Self {
        match value {
            0 => SimulationTickKind::Day,
            1 => SimulationTickKind::Week,
            2 => SimulationTickKind::Fortnight,
            3 => SimulationTickKind::Month,
            4 => SimulationTickKind::BiMonth,
            5 => SimulationTickKind::QuarterYear,
            6 => SimulationTickKind::HalfYear,
            7 => SimulationTickKind::Year,
            _ => panic!("Invalid simulation tick kind"),
        }
    }
}

impl Into<u8> for SimulationTickKind {
    fn into(self) -> u8 {
        self as u8
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

#[test]
fn from_into_simulation_tick_kind() {
    for i in 0..SIMULATION_TICK_KIND_AMOUNT {
        assert_eq!(
            SimulationTickKind::from(i),
            SimulationTickKind::from(i as u8)
        );
        assert_eq!(
            SimulationTickKind::from(i as u8),
            SimulationTickKind::from(i)
        );
    }
}
