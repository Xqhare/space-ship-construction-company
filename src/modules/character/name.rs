use ilmarinen::{MintingObjectType, ObjectResult};

use crate::{error::SSCCResult, utils::Utils};

pub struct CharacterName {
    legal_name: String,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    nick_name: String,
}

impl CharacterName {
    pub fn new(utils: &mut Utils) -> SSCCResult<Self> {
        let pers = utils
            .word_smith
            .mint_managed_object(&mut utils.rng, MintingObjectType::PeopleObject(vec![], 0))
            .map_err(|e| e.to_nemesis_error())?;
        let pers = match pers {
            ObjectResult::PeopleObject(p) => p,
            _ => unreachable!("person minter must return a person object"),
        };
        Ok(CharacterName {
            legal_name: pers.legal_name,
            first_name: pers.first_name,
            middle_name: pers.middle_name,
            last_name: pers.last_name,
            nick_name: pers.nick_name,
        })
    }
}
