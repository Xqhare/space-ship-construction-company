use std::path::Path;

use aequa::Object;
use athena::{float::HpFloat, slot_map::SlotMap};
use horae::Utc;
use tyche::RngApi;

use crate::{
    consts::characters::r#trait::{CharacterTrait, registry::CharacterTraitRegistryId},
    error::{SSCCResult, new_character_error, new_generation_error},
    modules::character::name::CharacterName,
    runtime::DataBoxSlotId,
    utils::{
        Utils,
        boxes::data_box::{DataBox, flag_box::FlagBox, stat_box::StatBox},
    },
};

pub mod name;

pub struct Character {
    pub name: CharacterName,
    /// The character's data box
    data_box: DataBoxSlotId,
    pub birthday: Utc,
    pub teams: Vec<TeamBoxSlotId>,
    /// The character's traits.
    /// For lookup purposes.
    /// All traits must be unique.
    traits: Vec<CharacterTraitRegistryId>,
    skill: Vec<CharacterSkill>,
}

impl Character {
    pub fn new_random(utils: &mut Utils, final_company_rep: HpFloat) -> SSCCResult<Character> {
        todo!()
    }
    /// Generates a new random character with one 'ultra' level skill.
    pub fn new_random_ultra_rare(
        utils: &mut Utils,
        final_company_rep: HpFloat,
    ) -> SSCCResult<Character> {
        todo!()
    }
    pub fn new_custom(
        utils: &mut Utils,
        final_company_rep: HpFloat,
        traits: Vec<CharacterTraitRegistryId>,
        skills: Vec<CharacterSkillRegistryId>,
        birth_day: Utc,
        name: Option<CharacterName>,
    ) -> SSCCResult<Character> {
        todo!()
    }
    pub fn data_box_flag_box<'sm>(
        &self,
        data_box_slot_map: &'sm mut SlotMap<DataBox>,
    ) -> SSCCResult<&'sm mut FlagBox> {
        if let Some(data_box) = data_box_slot_map.get_mut(self.data_box) {
            Ok(&mut data_box.flags)
        } else {
            Err(new_character_error("DataBox no longer exists")
                .add_ctx(format!("DataBox {:?} no longer exists", self.data_box)))
        }
    }
    /// Returns a mutable reference to the character's StatBox
    ///
    /// DO NOT USE THIS TO CALL `StatBox::get_full_stats`.
    /// Use `Character::get_full_stats` instead for skills to apply.
    pub fn data_box_stat_box<'sm>(
        &self,
        data_box_slot_map: &'sm mut SlotMap<DataBox>,
    ) -> SSCCResult<&'sm mut StatBox> {
        if let Some(data_box) = data_box_slot_map.get_mut(self.data_box) {
            Ok(&mut data_box.stats)
        } else {
            Err(new_character_error("DataBox no longer exists")
                .add_ctx(format!("DataBox {:?} no longer exists", self.data_box)))
        }
    }
    pub fn get_full_stats(
        &mut self,
        data_box_slot_map: &mut SlotMap<DataBox>,
    ) -> SSCCResult<Object> {
        if let Some(data_box) = data_box_slot_map.get_mut(self.data_box) {
            let mut stats = data_box.stats.get_full_stats();
            for skill in self.skill.iter_mut() {
                // TODO: Skill advancement through use
                // TODO: Apply skill to relevant stats
            }
            Ok(stats)
        } else {
            Err(new_character_error("DataBox no longer exists")
                .add_ctx(format!("DataBox {:?} no longer exists", self.data_box)))
        }
    }
    pub fn add_trait(&mut self, trait_id: CharacterTraitRegistryId) -> SSCCResult<()> {
        // Add trait to DataBox.
        // How exactly the traits values interact with the character's base values is yet
        // to be determined, but will be defined in this function
        todo!()
    }
    pub fn add_skill(&mut self, skill_id: CharacterSkillRegistryId) -> SSCCResult<()> {
        // Add skill to DataBox
        // First convert data from registry to a character specific struct
        //
        // The struct will ONLY track what is needed for skill advancement
        todo!()
    }
}
