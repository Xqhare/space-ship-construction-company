//! Basic Design:
//!
//! Tree like structure.
//! One root.
//! Fields are in Tiers.
//! Each Tier (except root) has parents (the tier above) and children (the tier below).
//! E.g: Logic -> Mathematics -> Physics -> BioPhysics -> Engineering
//!
//! Each field will have a current level and a name.
//! The level will be in years and the name will be the name of the field.
//! Starting off each field will have a level of `start_year` (e.g: 2500).
//!
//! Each field will be limited to be at most the same level as any parent.
//!
//! When an engineer does research, points will be allocated to each parent too, getting smaller
//! the further down the tree they are.
//! E.g: 100 Engineering -> 75 BioPhysics -> 50 Physics -> 25 Mathematics -> 10 Logic
//!
//! Cost will go up the further ahead in time the field is.
//! If the field level == current_year, there will always be the same value in cost / points.
//! Being behind will give bonuses, up to a point; When more than X years behing, you cannot fall
//! behind more (e.g: 10-15 years behind).
use aequa::{Object, XffSerde, XffValue, graph::Graph, xff};

use crate::{
    consts::science::research_fields::{
        t0::Logic,
        t1::Formal,
        t2::{Natural, Social},
        t3::Applied,
        t4::Disciplines,
        t5::SubDisciplines,
    },
    error::new_xff_serde_error,
};

pub mod t0;
pub mod t1;
pub mod t2;
pub mod t3;
pub mod t4;
pub mod t5;

pub struct ResearchField {
    pub name: String,
    pub kind: ResearchFieldKind,
}

pub enum ResearchFieldKind {
    Logic(Logic),
    Formal(Formal),
    Natural(Natural),
    Social(Social),
    Applied(Applied),
    Discipline(Disciplines),
    SubDiscipline(SubDisciplines),
}

impl XffSerde for ResearchFieldKind {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        match self {
            Self::Logic(x) => obj.insert("logic", x.to_xff()),
            Self::Formal(x) => obj.insert("formal", x.to_xff()),
            Self::Natural(x) => obj.insert("natural", x.to_xff()),
            Self::Social(x) => obj.insert("social", x.to_xff()),
            Self::Applied(x) => obj.insert("applied", x.to_xff()),
            Self::Discipline(x) => obj.insert("discipline", x.to_xff()),
            Self::SubDiscipline(x) => obj.insert("sub_discipline", x.to_xff()),
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(logic) = obj.get("logic") {
                Ok(ResearchFieldKind::Logic(Logic::from_xff(logic)?))
            } else if let Some(formal) = obj.get("formal") {
                Ok(ResearchFieldKind::Formal(Formal::from_xff(formal)?))
            } else if let Some(natural) = obj.get("natural") {
                Ok(ResearchFieldKind::Natural(Natural::from_xff(natural)?))
            } else if let Some(social) = obj.get("social") {
                Ok(ResearchFieldKind::Social(Social::from_xff(social)?))
            } else if let Some(applied) = obj.get("applied") {
                Ok(ResearchFieldKind::Applied(Applied::from_xff(applied)?))
            } else if let Some(discipline) = obj.get("discipline") {
                Ok(ResearchFieldKind::Discipline(Disciplines::from_xff(
                    discipline,
                )?))
            } else if let Some(sub_discipline) = obj.get("sub_discipline") {
                Ok(ResearchFieldKind::SubDiscipline(SubDisciplines::from_xff(
                    sub_discipline,
                )?))
            } else {
                Err(
                    new_xff_serde_error("Unable to load ResearchFieldKind from XFF")
                        .add_ctx(format!("Value: {xff:?}")),
                )
            }
        } else {
            Err(
                new_xff_serde_error("Unable to load ResearchFieldKind from XFF, not an object")
                    .add_ctx(format!("Value: {xff:?}")),
            )
        }
    }
}

pub fn research_field_graph() -> Graph {
    let metadata = XffValue::Null;
    let mut graph = Graph::new();
    // --- T0 ---
    let root_id = graph.add_node(
        ResearchFieldKind::Logic(Logic::Logic).to_xff(),
        metadata.clone(),
    );

    // --- T1 ---
    let mathematics_id = graph.add_node(
        ResearchFieldKind::Formal(Formal::Mathematics).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(root_id, mathematics_id, metadata.clone());
    let philosophy_id = graph.add_node(
        ResearchFieldKind::Formal(Formal::Philosophy).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(root_id, philosophy_id, metadata.clone());
    let empiricism_id = graph.add_node(
        ResearchFieldKind::Formal(Formal::Empiricism).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(root_id, empiricism_id, metadata.clone());

    // --- T2: Natural ---
    let nat_physics_id = graph.add_node(
        ResearchFieldKind::Natural(Natural::Physics).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(mathematics_id, nat_physics_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, nat_physics_id, metadata.clone());
    let nat_biology_id = graph.add_node(
        ResearchFieldKind::Natural(Natural::Biology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(mathematics_id, nat_biology_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, nat_biology_id, metadata.clone());
    let nat_chemistry_id = graph.add_node(
        ResearchFieldKind::Natural(Natural::Chemistry).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(mathematics_id, nat_chemistry_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, nat_chemistry_id, metadata.clone());
    let nat_astronomy_id = graph.add_node(
        ResearchFieldKind::Natural(Natural::Astronomy).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(mathematics_id, nat_astronomy_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, nat_astronomy_id, metadata.clone());
    let nat_geo_science_id = graph.add_node(
        ResearchFieldKind::Natural(Natural::GeoScience).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(mathematics_id, nat_geo_science_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, nat_geo_science_id, metadata.clone());
    // --- T2: Social ---
    let soc_psychology_id = graph.add_node(
        ResearchFieldKind::Social(Social::Psychology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_psychology_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_psychology_id, metadata.clone());
    let soc_economics_id = graph.add_node(
        ResearchFieldKind::Social(Social::Economics).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_economics_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_economics_id, metadata.clone());
    let _con_id = graph.add_connection(mathematics_id, soc_economics_id, metadata.clone());
    let soc_management_id = graph.add_node(
        ResearchFieldKind::Social(Social::Management).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_management_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_management_id, metadata.clone());
    let soc_communication_id = graph.add_node(
        ResearchFieldKind::Social(Social::Communication).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_communication_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_communication_id, metadata.clone());
    let soc_sociology_id = graph.add_node(
        ResearchFieldKind::Social(Social::Sociology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_sociology_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_sociology_id, metadata.clone());
    let soc_culturology_id = graph.add_node(
        ResearchFieldKind::Social(Social::Culturology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_culturology_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_culturology_id, metadata.clone());

    let soc_history_id = graph.add_node(
        ResearchFieldKind::Social(Social::History).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(philosophy_id, soc_history_id, metadata.clone());
    let _con_id = graph.add_connection(empiricism_id, soc_history_id, metadata.clone());

    // --- T3 ---
    let app_meterology_id = graph.add_node(
        ResearchFieldKind::Applied(Applied::Meterology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(nat_physics_id, app_meterology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_geo_science_id, app_meterology_id, metadata.clone());
    let app_geology_id = graph.add_node(
        ResearchFieldKind::Applied(Applied::Geology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(nat_physics_id, app_geology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_geo_science_id, app_geology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_chemistry_id, app_geology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_biology_id, app_geology_id, metadata.clone());
    let app_optics_id = graph.add_node(
        ResearchFieldKind::Applied(Applied::Optics).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(nat_physics_id, app_optics_id, metadata.clone());
    let _con_id = graph.add_connection(nat_chemistry_id, app_optics_id, metadata.clone());
    let app_geo_chemistry_id = graph.add_node(
        ResearchFieldKind::Applied(Applied::GeoChemistry).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(nat_astronomy_id, app_geo_chemistry_id, metadata.clone());
    let _con_id = graph.add_connection(nat_geo_science_id, app_geo_chemistry_id, metadata.clone());
    let _con_id = graph.add_connection(nat_chemistry_id, app_geo_chemistry_id, metadata.clone());
    let app_planetology_id = graph.add_node(
        ResearchFieldKind::Applied(Applied::Planetology).to_xff(),
        metadata.clone(),
    );
    let _con_id = graph.add_connection(nat_astronomy_id, app_planetology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_geo_science_id, app_planetology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_chemistry_id, app_planetology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_biology_id, app_planetology_id, metadata.clone());
    let _con_id = graph.add_connection(nat_physics_id, app_planetology_id, metadata.clone());
    // TODO: Rest of the already defined fields
    graph
}
