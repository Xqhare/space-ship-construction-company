pub const SCI_BONUS_ALL_FIELDS: (&'static str, f32) = ("bonus_all_fields", 0.0);

pub const MAX_SCIENCE_FIELD_PARENTS: usize = 6;

pub const fn has_sci_prefix(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 4 && b[0] == b's' && b[1] == b'c' && b[2] == b'i' && b[3] == b'_'
}

pub const fn ensure_sci_prefix(raw: &'static str, prefixed: &'static str) -> &'static str {
    if has_sci_prefix(raw) { raw } else { prefixed }
}

macro_rules! define_science_registry {
    (
        pub static $registry_name:ident = [
            $(
                $field_const:ident => ConstScienceField::$ctor:ident($name:literal)
                    $( .$method:ident($($args:tt)*) )*
            ),* $(,)?
        ];
    ) => {
        $(
            pub static $field_const: $crate::consts::base_values::science::ConstScienceField =
                $crate::consts::base_values::science::ConstScienceField::$ctor(
                    $crate::consts::base_values::science::ensure_sci_prefix(
                        $name,
                        concat!("sci_", $name),
                    )
                )
                $( .$method($($args)*) )*;
        )*

        pub static $registry_name: &[&$crate::consts::base_values::science::ConstScienceField] = &[
            $( &$field_const ),*
        ];
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScienceFieldKind {
    T0Logic,
    T1Formal,
    T2Natural,
    T2Social,
    T3Applied,
    T4Discipline,
    T5SubDiscipline,
}

impl ScienceFieldKind {
    pub const fn tier(&self) -> u8 {
        match self {
            Self::T0Logic => 0,
            Self::T1Formal => 1,
            Self::T2Natural | Self::T2Social => 2,
            Self::T3Applied => 3,
            Self::T4Discipline => 4,
            Self::T5SubDiscipline => 5,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct ConstScienceField {
    pub name: &'static str,
    pub kind: ScienceFieldKind,
    active_parents_count: usize,
    parents: [&'static str; MAX_SCIENCE_FIELD_PARENTS],
}

impl PartialEq for ConstScienceField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind
    }
}

impl Eq for ConstScienceField {}

impl ConstScienceField {
    pub const fn t0_logic(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T0Logic)
    }
    pub const fn t1_formal(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T1Formal)
    }
    pub const fn t2_natural(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T2Natural)
    }
    pub const fn t2_social(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T2Social)
    }
    pub const fn t3_applied(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T3Applied)
    }
    pub const fn t4_discipline(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T4Discipline)
    }
    pub const fn t5_sub_discipline(name: &'static str) -> Self {
        Self::new(name, ScienceFieldKind::T5SubDiscipline)
    }

    const fn new(name: &'static str, kind: ScienceFieldKind) -> Self {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            has_sci_prefix(name) && name.len() >= 5,
            "Science field name must start with 'sci_' followed by at least 1 character"
        );
        Self {
            name,
            kind,
            active_parents_count: 0,
            parents: [""; MAX_SCIENCE_FIELD_PARENTS],
        }
    }

    pub const fn with_parents<const N: usize>(
        mut self,
        parents: [&'static ConstScienceField; N],
    ) -> Self {
        // This should only be ever called at compile time; no runtime panics possible
        assert!(
            N <= MAX_SCIENCE_FIELD_PARENTS,
            "Science field has too many parents"
        );
        let mut i = 0;
        while i < N {
            assert!(
                parents[i].kind.tier() < self.kind.tier(),
                "Parent science field must be from a strictly lower tier"
            );
            self.parents[i] = parents[i].name;
            i += 1;
        }
        self.active_parents_count = N;
        self
    }

    pub fn parents(&self) -> &[&'static str] {
        &self.parents[..self.active_parents_count]
    }
}

pub static SCI_REGISTRY_OF_REGISTRIES: [&[&ConstScienceField]; 7] = [
    SCI_T0_REGISTRY,
    SCI_T1_FORMAL_REGISTRY,
    SCI_T2_NATURAL_REGISTRY,
    SCI_T2_SOCIAL_REGISTRY,
    SCI_T3_APPLIED_REGISTRY,
    SCI_T4_DISCIPLINE_REGISTRY,
    SCI_T5_SUB_DISCIPLINE_REGISTRY,
];

pub fn get_science_field_from_registry_by_name(
    field_name: &str,
) -> Option<&'static ConstScienceField> {
    SCI_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .find(|entry| entry.name == field_name)
}

define_science_registry! {
    pub static SCI_T0_REGISTRY = [
        SCI_T0_LOGIC => ConstScienceField::t0_logic("sci_t0_logic"),
    ];
}

define_science_registry! {
    pub static SCI_T1_FORMAL_REGISTRY = [
        SCI_T1_MATHEMATICS => ConstScienceField::t1_formal("sci_t1_mathematics")
            .with_parents([&SCI_T0_LOGIC]),
        SCI_T1_PHILOSOPHY => ConstScienceField::t1_formal("sci_t1_philosophy")
            .with_parents([&SCI_T0_LOGIC]),
        SCI_T1_EMPIRICISM => ConstScienceField::t1_formal("sci_t1_empiricism")
            .with_parents([&SCI_T0_LOGIC]),
    ];
}

define_science_registry! {
    pub static SCI_T2_NATURAL_REGISTRY = [
        SCI_T2_BIOLOGY => ConstScienceField::t2_natural("sci_t2_biology")
            .with_parents([&SCI_T1_MATHEMATICS, &SCI_T1_EMPIRICISM]),
        SCI_T2_PHYSICS => ConstScienceField::t2_natural("sci_t2_physics")
            .with_parents([&SCI_T1_MATHEMATICS, &SCI_T1_EMPIRICISM]),
        SCI_T2_CHEMISTRY => ConstScienceField::t2_natural("sci_t2_chemistry")
            .with_parents([&SCI_T1_MATHEMATICS, &SCI_T1_EMPIRICISM]),
        SCI_T2_ASTRONOMY => ConstScienceField::t2_natural("sci_t2_astronomy")
            .with_parents([&SCI_T1_MATHEMATICS, &SCI_T1_EMPIRICISM]),
        SCI_T2_GEOSCIENCE => ConstScienceField::t2_natural("sci_t2_geoscience")
            .with_parents([&SCI_T1_MATHEMATICS, &SCI_T1_EMPIRICISM]),
    ];
}

define_science_registry! {
    pub static SCI_T2_SOCIAL_REGISTRY = [
        SCI_T2_PSYCHOLOGY => ConstScienceField::t2_social("sci_t2_psychology")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
        SCI_T2_ECONOMICS => ConstScienceField::t2_social("sci_t2_economics")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM, &SCI_T1_MATHEMATICS]),
        SCI_T2_MANAGEMENT => ConstScienceField::t2_social("sci_t2_management")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
        SCI_T2_COMMUNICATION => ConstScienceField::t2_social("sci_t2_communication")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
        SCI_T2_SOCIOLOGY => ConstScienceField::t2_social("sci_t2_sociology")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
        SCI_T2_CULTUROLOGY => ConstScienceField::t2_social("sci_t2_culturology")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
        SCI_T2_HISTORY => ConstScienceField::t2_social("sci_t2_history")
            .with_parents([&SCI_T1_PHILOSOPHY, &SCI_T1_EMPIRICISM]),
    ];
}

define_science_registry! {
    pub static SCI_T3_APPLIED_REGISTRY = [
        SCI_T3_METEOROLOGY => ConstScienceField::t3_applied("sci_t3_meteorology")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_GEOSCIENCE]),
        SCI_T3_GEOLOGY => ConstScienceField::t3_applied("sci_t3_geology")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_GEOSCIENCE, &SCI_T2_CHEMISTRY, &SCI_T2_BIOLOGY]),
        SCI_T3_OPTICS => ConstScienceField::t3_applied("sci_t3_optics")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_CHEMISTRY]),
        SCI_T3_GEO_CHEMISTRY => ConstScienceField::t3_applied("sci_t3_geo_chemistry")
            .with_parents([&SCI_T2_ASTRONOMY, &SCI_T2_GEOSCIENCE, &SCI_T2_CHEMISTRY]),
        SCI_T3_PLANETOLOGY => ConstScienceField::t3_applied("sci_t3_planetology")
            .with_parents([&SCI_T2_ASTRONOMY, &SCI_T2_GEOSCIENCE, &SCI_T2_CHEMISTRY, &SCI_T2_BIOLOGY, &SCI_T2_PHYSICS]),
        SCI_T3_MATERIALS => ConstScienceField::t3_applied("sci_t3_materials")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_CHEMISTRY]),
        SCI_T3_COGNITIVE_SCIENCE => ConstScienceField::t3_applied("sci_t3_cognitive_science")
            .with_parents([&SCI_T2_PSYCHOLOGY, &SCI_T2_BIOLOGY]),
        SCI_T3_LAW => ConstScienceField::t3_applied("sci_t3_law")
            .with_parents([&SCI_T2_SOCIOLOGY, &SCI_T2_HISTORY]),
        SCI_T3_BIO_PHYSICS => ConstScienceField::t3_applied("sci_t3_bio_physics")
            .with_parents([&SCI_T2_BIOLOGY, &SCI_T2_PHYSICS]),
        SCI_T3_ECOLOGY => ConstScienceField::t3_applied("sci_t3_ecology")
            .with_parents([&SCI_T2_BIOLOGY, &SCI_T2_GEOSCIENCE]),
        SCI_T3_MINERALOGY => ConstScienceField::t3_applied("sci_t3_mineralogy")
            .with_parents([&SCI_T2_GEOSCIENCE, &SCI_T2_CHEMISTRY]),
        SCI_T3_CHEMICAL_PHYSICS => ConstScienceField::t3_applied("sci_t3_chemical_physics")
            .with_parents([&SCI_T2_CHEMISTRY, &SCI_T2_PHYSICS]),
        SCI_T3_ASTRO_CHEMISTRY => ConstScienceField::t3_applied("sci_t3_astro_chemistry")
            .with_parents([&SCI_T2_ASTRONOMY, &SCI_T2_CHEMISTRY]),
        SCI_T3_ASTRO_BIOLOGY => ConstScienceField::t3_applied("sci_t3_astro_biology")
            .with_parents([&SCI_T2_ASTRONOMY, &SCI_T2_BIOLOGY]),
        SCI_T3_KINETICS => ConstScienceField::t3_applied("sci_t3_kinetics")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_CHEMISTRY]),
        SCI_T3_EDUCATION => ConstScienceField::t3_applied("sci_t3_education")
            .with_parents([&SCI_T2_PSYCHOLOGY, &SCI_T2_COMMUNICATION, &SCI_T2_SOCIOLOGY]),
        SCI_T3_MILITARY => ConstScienceField::t3_applied("sci_t3_military")
            .with_parents([&SCI_T2_HISTORY, &SCI_T2_MANAGEMENT, &SCI_T2_PSYCHOLOGY]),
        SCI_T3_MEDICINE => ConstScienceField::t3_applied("sci_t3_medicine")
            .with_parents([&SCI_T2_BIOLOGY, &SCI_T2_CHEMISTRY]),
        SCI_T3_GEO_PHYSICS => ConstScienceField::t3_applied("sci_t3_geo_physics")
            .with_parents([&SCI_T2_GEOSCIENCE, &SCI_T2_PHYSICS]),
        SCI_T3_QUANTUM_PHYSICS => ConstScienceField::t3_applied("sci_t3_quantum_physics")
            .with_parents([&SCI_T2_PHYSICS]),
        SCI_T3_THERMODYNAMICS => ConstScienceField::t3_applied("sci_t3_thermodynamics")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_CHEMISTRY]),
        SCI_T3_BIO_CHEMISTRY => ConstScienceField::t3_applied("sci_t3_bio_chemistry")
            .with_parents([&SCI_T2_BIOLOGY, &SCI_T2_CHEMISTRY]),
        SCI_T3_ASTRO_PHYSICS => ConstScienceField::t3_applied("sci_t3_astro_physics")
            .with_parents([&SCI_T2_ASTRONOMY, &SCI_T2_PHYSICS]),
        SCI_T3_NANO_SCIENCE => ConstScienceField::t3_applied("sci_t3_nano_science")
            .with_parents([&SCI_T2_PHYSICS, &SCI_T2_CHEMISTRY]),
        SCI_T3_ANTHROPOLOGY => ConstScienceField::t3_applied("sci_t3_anthropology")
            .with_parents([&SCI_T2_CULTUROLOGY, &SCI_T2_HISTORY, &SCI_T2_BIOLOGY, &SCI_T2_SOCIOLOGY]),
        SCI_T3_ENVIRONMENTAL_SCIENCE => ConstScienceField::t3_applied("sci_t3_environmental_science")
            .with_parents([&SCI_T2_GEOSCIENCE, &SCI_T2_BIOLOGY, &SCI_T2_CHEMISTRY]),
        SCI_T3_ARCHEOLOGY => ConstScienceField::t3_applied("sci_t3_archeology")
            .with_parents([&SCI_T2_HISTORY, &SCI_T2_CULTUROLOGY, &SCI_T2_GEOSCIENCE]),
    ];
}

define_science_registry! {
    pub static SCI_T4_DISCIPLINE_REGISTRY = [
        SCI_T4_ENGINEERING => ConstScienceField::t4_discipline("sci_t4_engineering")
            .with_parents([&SCI_T3_MATERIALS, &SCI_T3_KINETICS, &SCI_T3_THERMODYNAMICS]),
        SCI_T4_CORPORATE_LAW => ConstScienceField::t4_discipline("sci_t4_corporate_law")
            .with_parents([&SCI_T3_LAW]),
    ];
}

define_science_registry! {
    pub static SCI_T5_SUB_DISCIPLINE_REGISTRY = [
        SCI_T5_COMPUTER_SCIENCE => ConstScienceField::t5_sub_discipline("sci_t5_computer_science")
            .with_parents([&SCI_T4_ENGINEERING]),
        SCI_T5_ARTIFICIAL_INTELLIGENCE => ConstScienceField::t5_sub_discipline("sci_t5_artificial_intelligence")
            .with_parents([&SCI_T4_ENGINEERING]),
        SCI_T5_AERO_SPACE_ENGINEERING => ConstScienceField::t5_sub_discipline("sci_t5_aero_space_engineering")
            .with_parents([&SCI_T4_ENGINEERING]),
    ];
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_science_fields_have_valid_parents_and_unique_names() {
        let mut seen_names = HashSet::new();

        for reg in SCI_REGISTRY_OF_REGISTRIES {
            for &field in reg {
                assert!(
                    field.name.starts_with("sci_") && field.name.len() >= 5,
                    "Science field '{}' does not have a valid 'sci_' prefix",
                    field.name
                );
                assert!(
                    seen_names.insert(field.name),
                    "Duplicate science field name '{}'",
                    field.name
                );

                if field.kind == ScienceFieldKind::T0Logic {
                    assert!(
                        field.parents().is_empty(),
                        "Root field '{}' should not have parents",
                        field.name
                    );
                } else {
                    assert!(
                        !field.parents().is_empty(),
                        "Science field '{}' must have at least one parent",
                        field.name
                    );
                }

                let mut seen_parents = HashSet::new();
                for &parent_name in field.parents() {
                    assert!(
                        seen_parents.insert(parent_name),
                        "Science field '{}' lists parent '{}' multiple times",
                        field.name,
                        parent_name
                    );
                    if let Some(parent) = get_science_field_from_registry_by_name(parent_name) {
                        assert!(
                            parent.kind.tier() < field.kind.tier(),
                            "Science field '{}' (tier {}) has parent '{}' of non-lower tier {}",
                            field.name,
                            field.kind.tier(),
                            parent.name,
                            parent.kind.tier()
                        );
                    } else {
                        panic!(
                            "Science field '{}' lists parent '{}', which does not exist in any registry",
                            field.name, parent_name
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn ensure_sci_prefix_adds_missing_prefix_without_doubling() {
        const ADDED: &str = ensure_sci_prefix("t1_mathematics", concat!("sci_", "t1_mathematics"));
        const KEPT: &str =
            ensure_sci_prefix("sci_t1_mathematics", concat!("sci_", "sci_t1_mathematics"));
        assert_eq!(ADDED, "sci_t1_mathematics");
        assert_eq!(KEPT, "sci_t1_mathematics");
    }
}
