use std::path::Path;

use horae::Utc;
use tyche::RngApi;

use crate::{
    consts::characters::r#trait::CharacterTrait,
    error::{SSCCResult, new_generation_error},
    modules::character::name::CharacterName,
    runtime::DataBoxSlotId,
    utils::Utils,
};

pub mod name;

pub struct Character<'char> {
    pub name: CharacterName,
    pub data_box: DataBoxSlotId,
    pub birthday: Utc,
    pub traits: Vec<&'char CharacterTrait>,
    pub skill: Vec<CharacterSkill>,
}

impl Character {
    pub fn new(
        utils: &mut Utils,
        trait_amount: u8,
        ultra_skill_amount: Option<u8>,
    ) -> SSCCResult<Character> {
        todo!()
    }
}
