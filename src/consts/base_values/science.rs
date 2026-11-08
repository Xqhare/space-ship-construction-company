pub type TbdPair = (&'static str, f32);

pub const SCI_BONUS_ALL_FIELDS: TbdPair = ("bonus_all_fields", 0.0);

// macro_rules! define_trait_registry {
//     (
//         pub static $registry_name:ident = [
//             $(
//                 $trait_const:ident => $trait_expr:expr
//             ),* $(,)?
//         ];
//     ) => {
//         $(
//             pub static $trait_const: $crate::consts::characters::traits::CharacterTrait = $trait_expr;
//         )*
//
//         pub static $registry_name: &[&$crate::consts::characters::traits::CharacterTrait] = &[
//             $( &$trait_const ),*
//         ];
//     };
// }

pub struct ConstScienceField {
    pub base_value_name: &'static str,
    pub base_value_value: f32,
}

impl ConstScienceField {}

/* This backend needs to work for at least two usecases:
 *
 * 1. Character Skills & Base values
 * 2. Science & Research system / module.
 *
 * First issue: The science and research system (in modules) doesnt work anymore really.
 * But I have my new and improved version in consts, but also unfinished.
 * To finish it, I would need these definitions done to query.
 *
 *
 *
 *
 * */

pub const SCI_ALL_FIELDS: [&[&TbdPair]; 5] = [SCI_T1_FORMAL];

pub const SCI_T0: TbdPair = SCI_T0_LOGIC;
pub const SCI_T0_LOGIC: TbdPair = ("sci_t0_logic", 0.0);

pub const SCI_T1_FORMAL: [TbdPair; 3] = [SCI_T1_MATHEMATICS, SCI_T1_PHILOSOPHY, SCI_T1_EMPIRICISM];
pub const SCI_T1_MATHEMATICS: TbdPair = ("sci_t1_mathematics", 0.0);
pub const SCI_T1_PHILOSOPHY: TbdPair = ("sci_t1_philosophy", 0.0);
pub const SCI_T1_EMPIRICISM: TbdPair = ("sci_t1_empiricism", 0.0);

pub const SCI_T2_NATURAL: [TbdPair; 5] = [
    SCI_T2_BIOLOGY,
    SCI_T2_PHYSICS,
    SCI_T2_CHEMISTRY,
    SCI_T2_ASTRONOMY,
    SCI_T2_GEOSCIENCE,
];
pub const SCI_T2_BIOLOGY: TbdPair = ("sci_t2_biology", 0.0);
pub const SCI_T2_PHYSICS: TbdPair = ("sci_t2_physics", 0.0);
pub const SCI_T2_CHEMISTRY: TbdPair = ("sci_t2_chemistry", 0.0);
pub const SCI_T2_ASTRONOMY: TbdPair = ("sci_t2_astronomy", 0.0);
pub const SCI_T2_GEOSCIENCE: TbdPair = ("sci_t2_geoscience", 0.0);
pub const SCI_T2_SOCIAL: [TbdPair; 7] = [
    SCI_T2_PSYCHOLOGY,
    SCI_T2_ECONOMICS,
    SCI_T2_MANGEMENT,
    SCI_T2_COMMUNICATION,
    SCI_T2_SOCIOLOGY,
    SCI_T2_CULTUROLOGY,
    SCI_T2_HISTORY,
];
pub const SCI_T2_PSYCHOLOGY: TbdPair = ("sci_t2_psychology", 0.0);
pub const SCI_T2_ECONOMICS: TbdPair = ("sci_t2_economics", 0.0);
pub const SCI_T2_MANGEMENT: TbdPair = ("sci_t2_mangement", 0.0);
pub const SCI_T2_COMMUNICATION: TbdPair = ("sci_t2_communication", 0.0);
pub const SCI_T2_SOCIOLOGY: TbdPair = ("sci_t2_sociology", 0.0);
pub const SCI_T2_CULTUROLOGY: TbdPair = ("sci_t2_culturology", 0.0);
pub const SCI_T2_HISTORY: TbdPair = ("sci_t2_history", 0.0);

pub const SCI_T3_APPLIED: [TbdPair; 20] = [
    SCI_T3_METEOROLOGY,
    SCI_T3_OPTICS,
    SCI_T3_MATERIALS,
    SCI_T3_BIO_PHYSICS,
    SCI_T3_CHEMICAL_PHYSICS,
    SCI_T3_ASTRO_CHEMISTRY,
    SCI_T3_ASTRO_BIOLOGY,
    SCI_T3_KINETICS,
    SCI_T3_EDUCATION,
    SCI_T3_MILITARY,
    SCI_T3_MEDICINE,
    SCI_T3_GEO_PHYSICS,
    SCI_T3_QUANTUM_PHYSICS,
    SCI_T3_THERMODYNAMICS,
    SCI_T3_BIO_CHEMISTRY,
    SCI_T3_ASTRO_PHYSICS,
    SCI_T3_NANO_SCIENCE,
    SCI_T3_ANTHROPOLOGY,
    SCI_T3_ENVIRONMENTAL_SCIENCE,
    SCI_T3_ARCHEOLOGY,
];
pub const SCI_T3_METEOROLOGY: TbdPair = ("sci_t3_meteorology", 0.0);
pub const SCI_T3_OPTICS: TbdPair = ("sci_t3_optics", 0.0);
pub const SCI_T3_MATERIALS: TbdPair = ("sci_t3_materials", 0.0);
pub const SCI_T3_BIO_PHYSICS: TbdPair = ("sci_t3_bio_physics", 0.0);
pub const SCI_T3_CHEMICAL_PHYSICS: TbdPair = ("sci_t3_chemical_physics", 0.0);
pub const SCI_T3_ASTRO_CHEMISTRY: TbdPair = ("sci_t3_astro_chemistry", 0.0);
pub const SCI_T3_ASTRO_BIOLOGY: TbdPair = ("sci_t3_astro_biology", 0.0);
pub const SCI_T3_KINETICS: TbdPair = ("sci_t3_kinetics", 0.0);
pub const SCI_T3_EDUCATION: TbdPair = ("sci_t3_education", 0.0);
pub const SCI_T3_MILITARY: TbdPair = ("sci_t3_military", 0.0);
pub const SCI_T3_MEDICINE: TbdPair = ("sci_t3_medicine", 0.0);
pub const SCI_T3_GEO_PHYSICS: TbdPair = ("sci_t3_geo_physics", 0.0);
pub const SCI_T3_QUANTUM_PHYSICS: TbdPair = ("sci_t3_quantum_physics", 0.0);
pub const SCI_T3_THERMODYNAMICS: TbdPair = ("sci_t3_thermodynamics", 0.0);
pub const SCI_T3_BIO_CHEMISTRY: TbdPair = ("sci_t3_bio_chemistry", 0.0);
pub const SCI_T3_ASTRO_PHYSICS: TbdPair = ("sci_t3_astronomy", 0.0);
pub const SCI_T3_NANO_SCIENCE: TbdPair = ("sci_t3_nano_science", 0.0);
pub const SCI_T3_ANTHROPOLOGY: TbdPair = ("sci_t3_antropology", 0.0);
pub const SCI_T3_ENVIRONMENTAL_SCIENCE: TbdPair = ("sci_t3_environmental_science", 0.0);
pub const SCI_T3_ARCHEOLOGY: TbdPair = ("sci_t3_archeology", 0.0);

#[test]
fn convention_check() {
    // In the end just a basic:
    all_sci.iter().for_each(sci.starts_with("sci"))
}
