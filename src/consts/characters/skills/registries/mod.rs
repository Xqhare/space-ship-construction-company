use crate::consts::characters::skills::ConstSkill;

pub mod social;

pub static SKILL_REGISTRY_OF_REGISTRIES: [&[&ConstSkill]; 0] = [];

pub fn get_trait_from_registry_by_name(trait_name: &str) -> Option<&'static ConstSkill> {
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

// TODO: Consider return type. Needs to be dynamically sized I think; Would be awesome if I didn't
// need to heap-alloc an output vec
pub fn get_secondary_skills_for_base_value(base_value_name: &str) -> Vec<&'static ConstSkill> {
    SKILL_REGISTRY_OF_REGISTRIES
        .iter()
        .flat_map(|reg| reg.iter())
        .copied()
        .filter_map(|entry| {
            if entry
                .secondary_skills()
                .iter()
                .any(|entry| entry.base_value_name == base_value_name)
            {
                Some(entry)
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_affecting_skills_from_registry(base_value_name: &str) -> Vec<&'static ConstSkill> {
    let mut out = get_secondary_skills_for_base_value(base_value_name);
    if let Some(master) = get_master_skill_for_base_value(base_value_name) {
        out.push(master);
    }
    out
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
