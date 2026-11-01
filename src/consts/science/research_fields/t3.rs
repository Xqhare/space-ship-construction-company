use aequa::{Object, XffSerde, XffValue, xff};

use crate::error::new_xff_serde_error;

pub enum Applied {
    Meterology,
    Geology,
    Optics,
    GeoChemistry,
    Planetology,
    Materials,
    CognitiveScience,
    Law,
    BioPhysics,
    Ecology,
    Mineralogy,
    ChemicalPhysics,
    AstroChemistry,
    AstroBiology,
    Kinetics,
    Education,
    Military,
    Medicine,
    Oceanography,
    GeoPhysics,
    QuantumPhysics,
    Thermodynamics,
    BioChemistry,
    Astrophysics,
    NanoScience,
    Anthropology,
    EnvironmentalScience,
    Archeology,
}

impl XffSerde for Applied {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Meterology => obj.insert("meterology", true),
            Self::Geology => obj.insert("geology", true),
            Self::Optics => obj.insert("optics", true),
            Self::GeoChemistry => obj.insert("geo_chemistry", true),
            Self::Planetology => obj.insert("planetology", true),
            Self::Materials => obj.insert("materials", true),
            Self::CognitiveScience => obj.insert("cognitive_science", true),
            Self::Law => obj.insert("law", true),
            Self::BioPhysics => obj.insert("bio_physics", true),
            Self::Ecology => obj.insert("ecology", true),
            Self::Mineralogy => obj.insert("mineralogy", true),
            Self::ChemicalPhysics => obj.insert("chemical_physics", true),
            Self::AstroChemistry => obj.insert("astro_chemistry", true),
            Self::AstroBiology => obj.insert("astro_biology", true),
            Self::Kinetics => obj.insert("kinetics", true),
            Self::Education => obj.insert("education", true),
            Self::Military => obj.insert("military", true),
            Self::Medicine => obj.insert("medicine", true),
            Self::Oceanography => obj.insert("oceanography", true),
            Self::GeoPhysics => obj.insert("geo_physics", true),
            Self::QuantumPhysics => obj.insert("quantum_physics", true),
            Self::Thermodynamics => obj.insert("thermodynamics", true),
            Self::BioChemistry => obj.insert("bio_chemistry", true),
            Self::Astrophysics => obj.insert("astrophysics", true),
            Self::NanoScience => obj.insert("nano_science", true),
            Self::Anthropology => obj.insert("anthropology", true),
            Self::EnvironmentalScience => obj.insert("environmental_science", true),
            Self::Archeology => obj.insert("archeology", true),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if obj.get("meterology").is_some() {
                Ok(Applied::Meterology)
            } else if obj.get("geology").is_some() {
                Ok(Applied::Geology)
            } else if obj.get("optics").is_some() {
                Ok(Applied::Optics)
            } else if obj.get("geo_chemistry").is_some() {
                Ok(Applied::GeoChemistry)
            } else if obj.get("planetology").is_some() {
                Ok(Applied::Planetology)
            } else if obj.get("materials").is_some() {
                Ok(Applied::Materials)
            } else if obj.get("cognitive_science").is_some() {
                Ok(Applied::CognitiveScience)
            } else if obj.get("law").is_some() {
                Ok(Applied::Law)
            } else if obj.get("bio_physics").is_some() {
                Ok(Applied::BioPhysics)
            } else if obj.get("ecology").is_some() {
                Ok(Applied::Ecology)
            } else if obj.get("mineralogy").is_some() {
                Ok(Applied::Mineralogy)
            } else if obj.get("chemical_physics").is_some() {
                Ok(Applied::ChemicalPhysics)
            } else if obj.get("astro_chemistry").is_some() {
                Ok(Applied::AstroChemistry)
            } else if obj.get("astro_biology").is_some() {
                Ok(Applied::AstroBiology)
            } else if obj.get("kinetics").is_some() {
                Ok(Applied::Kinetics)
            } else if obj.get("education").is_some() {
                Ok(Applied::Education)
            } else if obj.get("military").is_some() {
                Ok(Applied::Military)
            } else if obj.get("medicine").is_some() {
                Ok(Applied::Medicine)
            } else if obj.get("oceanography").is_some() {
                Ok(Applied::Oceanography)
            } else if obj.get("geo_physics").is_some() {
                Ok(Applied::GeoPhysics)
            } else if obj.get("quantum_physics").is_some() {
                Ok(Applied::QuantumPhysics)
            } else if obj.get("thermodynamics").is_some() {
                Ok(Applied::Thermodynamics)
            } else if obj.get("bio_chemistry").is_some() {
                Ok(Applied::BioChemistry)
            } else if obj.get("astrophysics").is_some() {
                Ok(Applied::Astrophysics)
            } else if obj.get("nano_science").is_some() {
                Ok(Applied::NanoScience)
            } else if obj.get("anthropology").is_some() {
                Ok(Applied::Anthropology)
            } else if obj.get("environmental_science").is_some() {
                Ok(Applied::EnvironmentalScience)
            } else if obj.get("archeology").is_some() {
                Ok(Applied::Archeology)
            } else {
                Err(new_xff_serde_error("Unable to load Applied from XFF"))
            }
        } else {
            Err(new_xff_serde_error(
                "Unable to load Applied from XFF, not an object",
            ))
        }
    }
}
