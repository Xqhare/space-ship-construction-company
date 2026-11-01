use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum Natural {
    Physics,
    Biology,
    Chemistry,
    Astronomy,
    GeoScience,
}

impl XffSerde for Natural {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Physics => obj.insert("physics", true),
            Self::Biology => obj.insert("biology", true),
            Self::Chemistry => obj.insert("chemistry", true),
            Self::Astronomy => obj.insert("astronomy", true),
            Self::GeoScience => obj.insert("geo_science", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("physics").is_some() {
                Ok(Natural::Physics)
            } else if obj.get("biology").is_some() {
                Ok(Natural::Biology)
            } else if obj.get("chemistry").is_some() {
                Ok(Natural::Chemistry)
            } else if obj.get("astronomy").is_some() {
                Ok(Natural::Astronomy)
            } else if obj.get("geo_science").is_some() {
                Ok(Natural::GeoScience)
            } else {
                Err(new_xff_serde_error("Unable to load Natural from XFF"))
            }
        } else {
            Err(new_xff_serde_error(
                "Unable to load Natural from XFF, not an object",
            ))
        }
    }
}

pub enum Social {
    Psychology,
    Economics,
    Management,
    Communication,
    Sociology,
    Culturology,
    History,
}

impl XffSerde for Social {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Psychology => obj.insert("psychology", true),
            Self::Economics => obj.insert("economics", true),
            Self::Management => obj.insert("management", true),
            Self::Communication => obj.insert("communication", true),
            Self::Sociology => obj.insert("sociology", true),
            Self::Culturology => obj.insert("culturology", true),
            Self::History => obj.insert("history", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("psychology").is_some() {
                Ok(Social::Psychology)
            } else if obj.get("economics").is_some() {
                Ok(Social::Economics)
            } else if obj.get("management").is_some() {
                Ok(Social::Management)
            } else if obj.get("communication").is_some() {
                Ok(Social::Communication)
            } else if obj.get("sociology").is_some() {
                Ok(Social::Sociology)
            } else if obj.get("culturology").is_some() {
                Ok(Social::Culturology)
            } else if obj.get("history").is_some() {
                Ok(Social::History)
            } else {
                Err(new_xff_serde_error("Unable to load Social from XFF"))
            }
        } else {
            Err(new_xff_serde_error(
                "Unable to load Social from XFF, not an object",
            ))
        }
    }
}
