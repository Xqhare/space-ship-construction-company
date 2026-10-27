use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;
use nemesis::NemesisResultExt;

use crate::{
    consts::durations::{DAY_DUR_SECS, FORTNIGHT_DUR_SECS, WEEK_DUR_SECS},
    error::new_xff_serde_error,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tick {
    Simulation(SimulationTick),
    Special(SpecialTick),
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

// NOTE: `SpecialTickKind` is limited by `EventQueue`'s bit-flag to 16 entries at most
pub const SPECIAL_TICK_KIND_AMOUNT: u16 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecialTick {
    pub time: Utc,
    pub kind: SpecialTickKind,
}

impl XffSerde for SpecialTick {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("time", self.time.to_xff());
        obj.insert("kind", Into::<u16>::into(self.kind));
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
            let kind = SpecialTickKind::from(kind as u16);
            Ok(SpecialTick { time, kind })
        } else {
            Err(new_xff_serde_error("Unable to load SpecialTick from XFF")
                .add_ctx(format!("Value: {xff:?}")))
        }
    }
}
impl SpecialTick {
    pub fn new_from_kind(now: Utc, special_tick_kind: SpecialTickKind) -> Self {
        let year = match special_tick_kind {
            SpecialTickKind::Biennial => 2,
            SpecialTickKind::Triennial => 3,
            SpecialTickKind::Quintennial => 5,
            SpecialTickKind::Decennial => 10,
            SpecialTickKind::Quadranscentennial => 25,
            SpecialTickKind::Semicentennial => 50,
            SpecialTickKind::Demisesquicentennial => 75,
            SpecialTickKind::Centennial => 100,
            SpecialTickKind::Quasquicentennial => 125,
            SpecialTickKind::Sesquicentennial => 150,
            SpecialTickKind::Dodransbicentennial => 175,
            SpecialTickKind::Bicentennial => 200,
            SpecialTickKind::Semiquincentennial => 250,
            SpecialTickKind::Semiseptcentennial => 300,
            SpecialTickKind::Quadricentennial => 400,
            SpecialTickKind::Quincentennial => 500,
        };
        let time = Utc::from_ymd_hms(now.date().year.saturating_add(year), 1, 1, 0, 0, 0);
        SpecialTick {
            time,
            kind: special_tick_kind,
        }
    }
}
/// Think of the ticks as special triggers for events mostly.
/// Simulation-wise, they are not that important as they are always paired with a simulation tick
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SpecialTickKind {
    /// Every 2 years
    Biennial = 0,
    /// Every 3 years
    Triennial = 1,
    /// Every 5 years
    Quintennial = 2,
    /// Every 10 years
    Decennial = 3,
    /// Every 25 years
    Quadranscentennial = 4,
    // NOTE: From here on, I expect at most one or two ticks of these kinds to happen in a long to very long game.
    // They are likely to trigger special events for dedicated players
    /// Every 50 years
    Semicentennial = 5,
    /// Every 75 years
    Demisesquicentennial = 6,
    /// Every 100 years
    Centennial = 7,
    /// Every 125 years
    Quasquicentennial = 8,
    // NOTE: I expect these kinds to almost never trigger in a save. They should be a reward for the most dedicated players
    /// Every 150 years
    Sesquicentennial = 9,
    /// Every 175 years
    Dodransbicentennial = 10,
    /// Every 200 years
    Bicentennial = 11,
    /// Every 250 years
    Semiquincentennial = 12,
    /// Every 300 years
    Semiseptcentennial = 13,
    /// Every 400 years
    Quadricentennial = 14,
    // NOTE: With the current start date of 2500, this would trigger on 01.01.3000
    /// Every 500 years (EU5 Equivalent; Technically a few days more)
    Quincentennial = 15,
}

impl From<u16> for SpecialTickKind {
    fn from(value: u16) -> Self {
        match value {
            0 => SpecialTickKind::Biennial,
            1 => SpecialTickKind::Triennial,
            2 => SpecialTickKind::Quintennial,
            3 => SpecialTickKind::Decennial,
            4 => SpecialTickKind::Quadranscentennial,
            5 => SpecialTickKind::Semicentennial,
            6 => SpecialTickKind::Demisesquicentennial,
            7 => SpecialTickKind::Centennial,
            8 => SpecialTickKind::Quasquicentennial,
            9 => SpecialTickKind::Sesquicentennial,
            10 => SpecialTickKind::Dodransbicentennial,
            11 => SpecialTickKind::Bicentennial,
            12 => SpecialTickKind::Semiquincentennial,
            13 => SpecialTickKind::Semiseptcentennial,
            14 => SpecialTickKind::Quadricentennial,
            15 => SpecialTickKind::Quincentennial,
            _ => panic!("Invalid tick kind"),
        }
    }
}

impl Into<u16> for SpecialTickKind {
    fn into(self) -> u16 {
        self as u16
    }
}

#[test]
fn from_into_special_tick_kind() {
    for i in 0..SPECIAL_TICK_KIND_AMOUNT {
        let tick_kind = SpecialTickKind::from(i);
        assert_eq!(Into::<u16>::into(tick_kind), i);
    }
}
