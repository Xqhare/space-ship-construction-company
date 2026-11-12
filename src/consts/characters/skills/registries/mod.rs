use crate::consts::characters::skills::{ConstSkill, registries::social::SKILL_SOCIAL_REGISTRY};

pub mod social;

pub static SKILL_REGISTRY_OF_REGISTRIES: [&[&ConstSkill]; 1] = [SKILL_SOCIAL_REGISTRY];

pub fn get_skill_from_registry_by_name(trait_name: &str) -> Option<&'static ConstSkill> {
    SKILL_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .find(|entry| entry.name == trait_name)
}

pub fn get_master_skill_for_base_value(base_value_name: &str) -> Option<&'static ConstSkill> {
    SKILL_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .find(|entry| {
            if let Some(master) = entry.master
                && master.base_value_name == base_value_name
            {
                true
            } else {
                false
            }
        })
}

pub fn get_secondary_skills_for_base_value<'a>(
    base_value_name: &'a str,
) -> impl Iterator<Item = &'static ConstSkill> + 'a {
    SKILL_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .filter(move |entry| {
            entry
                .secondary_skills()
                .iter()
                .any(|sec| sec.base_value_name == base_value_name)
        })
}

pub fn get_all_affecting_skills_from_registry<'a>(
    base_value_name: &'a str,
) -> impl Iterator<Item = &'static ConstSkill> + 'a {
    get_secondary_skills_for_base_value(base_value_name)
        .chain(get_master_skill_for_base_value(base_value_name))
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::consts::characters::skills::registries::SKILL_REGISTRY_OF_REGISTRIES;

    #[test]
    fn only_one_master_skill_per_base_value() {
        let mut tmp = HashMap::new();
        SKILL_REGISTRY_OF_REGISTRIES.iter().flat_map(|reg| reg.iter()).copied().for_each(|entry| {
            if let Some(master) = entry.master {
                let ret = tmp.insert(master.base_value_name, entry);
                if let Some(ret) = ret {
                    assert!(false, "Base_value '{}' has already defined a master skill '{ret:?}' but '{master:?}' is also a master of '{ret:?}'", master.base_value_name, )
                }
            }
        });
    }
    #[test]
    fn all_skill_names_unique() {
        let mut tmp = HashSet::new();
        SKILL_REGISTRY_OF_REGISTRIES
            .iter()
            .flat_map(|reg| reg.iter())
            .copied()
            .for_each(|entry| {
                assert!(
                    tmp.insert(entry.name),
                    "{} defined more than once!",
                    entry.name
                )
            })
    }
}
