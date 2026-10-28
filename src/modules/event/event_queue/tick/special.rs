use aequa::{Object, XffSerde, XffValue, xff};
use horae::Utc;
use nemesis::NemesisResultExt;

use crate::error::new_xff_serde_error;

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
    // NOTE: From here on, I expect a low single digit amount of ticks of these kinds to happen in a long to very long game.
    /// Every 10 years
    Decennial = 3,
    /// Every 25 years
    Quadranscentennial = 4,
    // They are likely to trigger special events for dedicated players
    /// Every 50 years
    Semicentennial = 5,
    /// Every 75 years
    Demisesquicentennial = 6,
    // NOTE: I expect these kinds to almost never trigger in a save. They should be a reward for the most dedicated players
    /// Every 100 years
    Centennial = 7,
    /// Every 125 years
    Quasquicentennial = 8,
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
    // NOTE: With the current start date of 2500, this would trigger on 3000.01.01
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
