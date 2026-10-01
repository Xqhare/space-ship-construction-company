pub mod research;
pub mod tech;

pub mod modifiers {
    pub enum PermanentModifierID {
        ShipModifier(u32),
        TechModifier(u32),
        GlobalModifier(u32),
    }
}

pub mod events {
    pub type EventID = u32;
}

pub type DateTime = u32;

mod tests {
    use aequa::{Array, Object, XffValue, xff};

    #[test]
    fn general_api() {
        let techs = Array::from(vec![xff!(Object::from(vec![
            ("id", xff!(0)),
            ("name", xff!("dummy tech")),
            ("description", xff!("dummy tech description")),
            ("kind", xff!("Hull")),
            ("cost", xff!(Object::from(vec![("points", xff!(10))]))),
            ("paid_cost", xff!(Object::from(vec![("points", xff!(1))]))),
            ("tech_prerequisites", xff!(vec![])),
            ("general_prerequisites", xff!(vec![])),
        ]))]);
        let science_module = SciModule::load(techs);
        assert!(science_module.is_ok());
        let science_module = science_module.unwrap();
        for tech in science_module.get_all_techs() {
            // vector of IDs
            println!("{:?}", tech);
        }
        for tech in science_module.get_researchable_techs() {
            // vector of IDs
            println!("{:?}", tech);
        }
        for tech in science_module.get_researched_techs() {
            // vector of IDs
            println!("{:?}", tech);
        }
        for tech in science_module.get_locked_techs() {
            // vector of IDs
            println!("{:?}", tech);
        }
        let tech = science_module.get_mut_tech_by_id(0);
        assert!(tech.is_ok());
        assert!(tech.unwrap().is_some());
        let tech = tech.unwrap().unwrap();
        assert!(!tech.is_locked());
        assert!(!tech.is_researched());
        assert!(tech.is_researchable());
        let researched = tech.inst_research(); // Research without paying any cost
        assert!(researched.is_ok());
        let unresearched = tech.unresarch();
        assert!(unresearched.is_ok());
        let cost = Object::from(vec![("points", xff!(5))]);
        let researched = tech.research(cost);
        assert!(researched.is_ok());
        assert!(!tech.is_researched());
        let researched = tech.research(cost).unwrap();
        assert!(
            researched
                .get("points")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap()
                == 1
        ); // 1 point over what was needed
        assert!(tech.is_researched());
    }
}
