use aequa::{Object, XffValue};

use crate::modules::science::{DateTime, events::EventID, modifiers::PermanentModifierID};

pub enum TechnologyKind {
    Engineering,
    AtmosphericFlight,
    ShipDesign,
    Propulsion,
    Navigation,
    Communication,
    Weapons,
    Sensors,
    Shield,
    Armor,
    Hull,
    Cargo,
    Crew,
    Power,
    LifeSupport,
    Maintenance,
}

impl TryFrom<&str> for TechnologyKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Engineering" => Ok(TechnologyKind::Engineering),
            "Atmospheric Flight" => Ok(TechnologyKind::AtmosphericFlight),
            "Ship Design" => Ok(TechnologyKind::ShipDesign),
            "Propulsion" => Ok(TechnologyKind::Propulsion),
            "Navigation" => Ok(TechnologyKind::Navigation),
            "Communication" => Ok(TechnologyKind::Communication),
            "Weapons" => Ok(TechnologyKind::Weapons),
            "Sensors" => Ok(TechnologyKind::Sensors),
            "Shield" => Ok(TechnologyKind::Shield),
            "Armor" => Ok(TechnologyKind::Armor),
            "Hull" => Ok(TechnologyKind::Hull),
            "Cargo" => Ok(TechnologyKind::Cargo),
            "Crew" => Ok(TechnologyKind::Crew),
            "Power" => Ok(TechnologyKind::Power),
            "Life Support" => Ok(TechnologyKind::LifeSupport),
            "Maintenance" => Ok(TechnologyKind::Maintenance),
            _ => Err(()),
        }
    }
}

impl ToString for TechnologyKind {
    fn to_string(&self) -> String {
        match self {
            TechnologyKind::Engineering => "Engineering",
            TechnologyKind::AtmosphericFlight => "Atmospheric Flight",
            TechnologyKind::ShipDesign => "Ship Design",
            TechnologyKind::Propulsion => "Propulsion",
            TechnologyKind::Navigation => "Navigation",
            TechnologyKind::Communication => "Communication",
            TechnologyKind::Weapons => "Weapons",
            TechnologyKind::Sensors => "Sensors",
            TechnologyKind::Shield => "Shield",
            TechnologyKind::Armor => "Armor",
            TechnologyKind::Hull => "Hull",
            TechnologyKind::Cargo => "Cargo",
            TechnologyKind::Crew => "Crew",
            TechnologyKind::Power => "Power",
            TechnologyKind::LifeSupport => "Life Support",
            TechnologyKind::Maintenance => "Maintenance",
        }
        .to_string()
    }
}

pub struct TechnologyCost {
    pub points: u32,
}

impl TryFrom<Object> for TechnologyCost {
    type Error = ();
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Object> for TechnologyCost {
    type Error = ();
    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let points: u32 = match value.get("points") {
            Some(XffValue::Number(num)) => match num.into_usize() {
                Some(points) => points as u32,
                None => return Err(()),
            },
            _ => return Err(()),
        };
        Ok(TechnologyCost { points })
    }
}

pub enum Prerequisite {
    PermanentModifier(PermanentModifierID),
    Event(EventID),
    DateTime(DateTime),
}

pub type TechnologyID = u32;

pub struct Technology {
    pub id: TechnologyID,
    pub name: String,
    pub description: String,
    pub kind: TechnologyKind,
    pub cost: TechnologyCost,
    pub paid_cost: TechnologyCost,
    pub tech_prerequisites: Vec<TechnologyID>,
    /// Prerequisites that are not technology
    /// E.g. Permanent Modifiers, Events, Race, Date/Time etc
    pub general_prerequisites: Vec<Prerequisite>,
}

impl TryFrom<Object> for Technology {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let id: TechnologyID = match value.get("id") {
            Some(XffValue::Number(num)) => match num.into_usize() {
                Some(id) => id as TechnologyID,
                None => return Err(()),
            },
            _ => return Err(()),
        };
        let name: String = match value.get("name") {
            Some(XffValue::String(name)) => name.to_string(),
            _ => return Err(()),
        };
        let description: String = match value.get("description") {
            Some(XffValue::String(description)) => description.to_string(),
            _ => return Err(()),
        };
        let kind: TechnologyKind = match value.get("kind") {
            Some(XffValue::String(kind)) => match TechnologyKind::try_from(kind.as_str()) {
                Ok(kind) => kind,
                Err(_) => return Err(()),
            },
            _ => return Err(()),
        };
        let cost: TechnologyCost = match value.get("cost") {
            Some(XffValue::Object(cost)) => match TechnologyCost::try_from(cost) {
                Ok(cost) => cost,
                Err(_) => return Err(()),
            },
            _ => return Err(()),
        };
        let paid_cost: TechnologyCost = match value.get("paid_cost") {
            Some(XffValue::Object(paid_cost)) => match TechnologyCost::try_from(paid_cost) {
                Ok(paid_cost) => paid_cost,
                Err(_) => return Err(()),
            },
            _ => return Err(()),
        };
        let tech_prerequisites: Vec<TechnologyID> = match value.get("tech_prerequisites") {
            Some(XffValue::Array(tech_prerequisites)) => match tech_prerequisites
                .iter()
                .map(|tech| match tech {
                    XffValue::Number(num) => match num.into_usize() {
                        Some(id) => Ok(id as TechnologyID),
                        None => Err(()),
                    },
                    _ => Err(()),
                })
                .collect()
            {
                Ok(tech_prerequisites) => tech_prerequisites,
                Err(_) => return Err(()),
            },
            _ => return Err(()),
        };
        let general_prerequisites: Vec<Prerequisite> = match value.get("general_prerequisites") {
            Some(XffValue::Array(_general_prerequisites)) =>
            // TODO
            {
                vec![]
            }
            _ => return Err(()),
        };
        Ok(Technology {
            id,
            name,
            description,
            kind,
            cost,
            paid_cost,
            tech_prerequisites,
            general_prerequisites,
        })
    }
}
