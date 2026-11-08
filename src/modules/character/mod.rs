use std::path::Path;

use aequa::Object;
use athena::{float::HpFloat, slot_map::SlotMap};
use horae::Utc;
use tyche::RngApi;

use crate::{
    consts::characters::traits::{
        CharacterTrait, CharacterTraitKind, registries::get_trait_from_registry_by_name,
    },
    error::{SSCCResult, new_character_error, new_generation_error},
    modules::character::{
        name::CharacterName,
        skill::{CharacterSkill, CharacterSkillRegistryId},
        r#trait::CharacterTraitStore,
    },
    utils::{
        Utils,
        boxes::{
            DataBoxSlotId, TeamBoxSlotId,
            data_box::{DataBox, flag_box::FlagBox, stat_box::StatBox},
        },
    },
};

pub mod name;
pub mod skill;
pub mod r#trait;

pub struct Character {
    pub name: CharacterName,
    /// The character's data box
    data_box: DataBoxSlotId,
    pub birthday: Utc,
    // TODO: Retirement System with national retirement age, company benefits etc. At some point -
    // Fixed date should work for a good while
    pub retirement: Utc,
    pub teams: Vec<TeamBoxSlotId>,
    /// The character's traits.
    /// All traits must be unique.
    traits: CharacterTraitStore,
    skill: Vec<CharacterSkill>,
}

impl Character {
    pub fn new_random(utils: &mut Utils, final_company_rep: HpFloat) -> SSCCResult<Character> {
        // Build a valid call to `self::new_custom`
        todo!()
    }
    /// Generates a new random character with one 'ultra' level skill.
    pub fn new_random_ultra_rare(
        utils: &mut Utils,
        final_company_rep: HpFloat,
    ) -> SSCCResult<Character> {
        // Build a valid call to `self::new_custom`
        todo!()
    }
    pub fn new_custom(
        utils: &mut Utils,
        final_company_rep: HpFloat,
        traits: Vec<CharacterTrait>,
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
    /// DO NOT USE THIS TO CALL [`StatBox::get_full_stats`].
    /// Use [`Character::get_full_stats`] instead for skills to apply.
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
    /// Remember to call [`Character::tick_skill_use`] if applicable!
    pub fn get_full_stats(&self, data_box_slot_map: &SlotMap<DataBox>) -> SSCCResult<Object> {
        if let Some(data_box) = data_box_slot_map.get(self.data_box) {
            let mut stats = data_box.stats.get_full_stats();
            self.apply_traits_to_stats(&mut stats);
            for skill in self.skill.iter() {
                // First get the base_value with the relevant modifiers applied;
                // Then modify that value by skill.
                // Meaning: Skill (even Low(0)) would always improve whatever output
                // there was. (except if stat box returns a 0. That could even be a critical failure? - Or be kind and at least give 0.001?)
                // TODO: Apply skill to relevant stats
            }
            Ok(stats)
        } else {
            Err(new_character_error("DataBox no longer exists")
                .add_ctx(format!("DataBox {:?} no longer exists", self.data_box)))
        }
    }
    fn apply_traits_to_stats(&self, stats: &mut Object) {
        for accu in self.traits.make_all_traits_accum() {
            let (stat_name, (flat_accu, perc_accu)) = accu;
            if let Some(stat) = stats.get_mut(&stat_name) {
                if let Some(stat) = stat.as_hp_float_mut() {
                    *stat = (*stat + flat_accu) * perc_accu;
                }
            }
        }
    }
    pub fn tick_skill_use(
        &mut self,
        data_box_slot_map: &mut SlotMap<DataBox>,
        used_base_values: &[&str],
        used_skills: Option<&[&str]>,
    ) -> SSCCResult<()> {
        if let Some(data_box) = data_box_slot_map.get_mut(self.data_box) {
            let mut stats = data_box.stats.get_full_stats();
            for skill in self.skill.iter_mut() {
                // TODO: Skill advancement through use
                // Firstly, the master skill associated with the used_base_value
                // (advancing it or adding it at level Low(0) if not present).
                // Then by the used skills (e.g. lying)
                //
                // So:
                // - used_bv for base_values with an associated Master skill
                // - used_skill for base_values without (e.g. Socialbility)
            }
            Ok(())
        } else {
            Err(new_character_error("DataBox no longer exists")
                .add_ctx(format!("DataBox {:?} no longer exists", self.data_box)))
        }
    }
    /// Add a trait via its name.
    ///
    /// # Errors
    /// Should the supplied trait name not exists in any registry.
    pub fn add_trait_via_name(
        &mut self,
        trait_name: &str,
        data_box_slot_map: &mut SlotMap<DataBox>,
    ) -> SSCCResult<()> {
        self.add_trait(
            get_trait_from_registry_by_name(trait_name)
                .ok_or(new_character_error("Can't add trait, name invalid"))?,
            data_box_slot_map,
        );
        Ok(())
    }
    /// Adds the supplied trait onto the character.
    ///
    /// Will only add the trait if an optional trait was not present, or if it is a `general`
    /// trait.
    ///
    /// Use [`Character::force_trait`] to force the supplied trait onto the character.
    pub fn add_trait(
        &mut self,
        character_trait: &'static CharacterTrait,
        data_box_slot_map: &mut SlotMap<DataBox>,
    ) {
        if self.traits.add_trait(character_trait) {
            self.add_flags(character_trait.flags(), data_box_slot_map);
        }
    }
    fn add_flags(&mut self, flags: &[&str], data_box_slot_map: &mut SlotMap<DataBox>) {
        for flag in flags {
            self.data_box_flag_box(data_box_slot_map)
                .expect("fix also")
                .add_flag(*flag);
        }
    }
    /// Forces the supplied new trait onto the character.
    ///
    /// Consider using [`Character::add_trait`]
    pub fn force_trait(
        &mut self,
        character_trait: &'static CharacterTrait,
        data_box_slot_map: &mut SlotMap<DataBox>,
    ) {
        character_trait
            .conflicting_traits()
            .iter()
            .filter_map(|conf| {
                if self.traits.has_trait(conf) {
                    Some(*conf)
                } else {
                    None
                }
            })
            .for_each(|conf| {
                self.remove_trait(
                    get_trait_from_registry_by_name(conf).expect("Make this safe"),
                    data_box_slot_map,
                )
            });
        self.traits.force_trait(character_trait);
        self.add_flags(character_trait.flags(), data_box_slot_map);
    }
    pub fn remove_trait(
        &mut self,
        character_trait: &'static CharacterTrait,
        data_box_slot_map: &mut SlotMap<DataBox>,
    ) {
        if let Some(r#trait) = self.traits.get_trait(character_trait.name) {
            if r#trait.kind == CharacterTraitKind::Intellect
                || r#trait.kind == CharacterTraitKind::Looks
            {
                // Cannot remove really, Only update via `force_trait`
            } else {
                for flag in character_trait.flags() {
                    self.data_box_flag_box(data_box_slot_map)
                        .expect("fix expect with error ret")
                        .remove_flag(*flag);
                }
                self.traits.remove_trait(r#trait);
            }
        }
    }
    pub fn add_skill(&mut self, skill_id: CharacterSkillRegistryId) -> SSCCResult<()> {
        // Add skill to DataBox
        // First convert data from registry to a character specific struct
        //
        // The struct will ONLY track what is needed for skill advancement
        todo!()
    }
}
