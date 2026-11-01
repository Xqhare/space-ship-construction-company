use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum SubDisciplines {
    ComputerScience,
    ArtificialIntelligence,
    AeroSpaceEngineering,
}

impl XffSerde for SubDisciplines {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::ComputerScience => obj.insert("computer_science", true),
            Self::ArtificialIntelligence => obj.insert("artificial_intelligence", true),
            Self::AeroSpaceEngineering => obj.insert("aero_space_engineering", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("computer_science").is_some() {
                Ok(SubDisciplines::ComputerScience)
            } else if obj.get("artificial_intelligence").is_some() {
                Ok(SubDisciplines::ArtificialIntelligence)
            } else if obj.get("aero_space_engineering").is_some() {
                Ok(SubDisciplines::AeroSpaceEngineering)
            } else {
                Err(new_xff_serde_error(
                    "Unable to load SubDisciplines from XFF",
                ))
            }
        } else {
            Err(new_xff_serde_error(
                "Unable to load SubDisciplines from XFF",
            ))
        }
    }
}
