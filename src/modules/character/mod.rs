use std::path::Path;

use ilmarinen::{MintingObjectType, ObjectResult, SkillLevel};
use tyche::RngApi;

use crate::{
    error::{SSCCResult, new_generation_error},
    utils::Utils,
};

// TODO: Need to add to Ilmarinen:
// For Mortality:
// 1. Age
// 2. Disease Resistance
//
// Health would be a base value with modifiers for age and disease resistance
pub struct Character {
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub skills: Vec<(String, String)>,
    pub traits: Vec<String>,
}

impl Character {
    pub fn new(
        utils: &mut Utils,
        trait_amount: u8,
        ultra_skill_amount: Option<u8>,
    ) -> SSCCResult<Character> {
        let skills = {
            let mut tmp: Vec<(u8, SkillLevel)> = Vec::with_capacity(32);
            if let Some(ultra_skill_amount) = ultra_skill_amount {
                tmp.push((ultra_skill_amount, SkillLevel::Ultra));
            }
            let low_range = utils
                .rng
                .random_from_range_inclusive(4, 12)
                .map_err(|e| e.to_nemesis_error())?;
            let medium_range = utils
                .rng
                .random_from_range_inclusive(1, 6)
                .map_err(|e| e.to_nemesis_error())?;
            tmp.push((low_range as u8, SkillLevel::Low));
            tmp.push((medium_range as u8, SkillLevel::Medium));
            if utils.rng.random_bool().map_err(|e| e.to_nemesis_error())? {
                let high_range = utils
                    .rng
                    .random_from_range_inclusive(0, 2)
                    .map_err(|e| e.to_nemesis_error())?;
                if high_range != 0 {
                    tmp.push((high_range as u8, SkillLevel::High));
                }
            }
            tmp
        };
        let people_obj = utils
            .word_smith
            .mint_managed_object(
                &mut utils.rng,
                MintingObjectType::PeopleObject(skills, trait_amount),
            )
            .map_err(|e| e.to_nemesis_error())?;
        let people_obj = match people_obj {
            ObjectResult::PeopleObject(p) => p,
            _ => {
                return Err(new_generation_error(
                    "Trying to generate new character. Unable to mint people object, returned type not PeopleObject.",
                ));
            }
        };
        let skills = people_obj.skills;
        let traits = people_obj.traits;
        let name = people_obj.legal_name;
        let first_name = people_obj.first_name;
        let last_name = people_obj.last_name;
        let nickname = people_obj.nick_name;
        Ok(Character {
            name,
            first_name,
            last_name,
            nickname,
            skills,
            traits,
        })
    }
}
