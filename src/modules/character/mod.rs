use std::{collections::HashMap, time::Duration};

use aequa::Object;
use athena::{float::HpFloat, slot_map::SlotMap};
use horae::Utc;
use tyche::RngApi;

use crate::{
    consts::{
        characters::{
            RETIREMENT_AGE,
            base_values::IQ,
            skills::{
                ConstSkill,
                levels::{Level, POINT_COST_LOW_LEVEL, SKILL_LEVEL_ULTRA_AMOUNT},
                registries::{
                    SKILL_REGISTRY_OF_REGISTRIES, get_master_skill_for_base_value,
                    get_skill_from_registry_by_name,
                },
            },
            traits::{
                CharacterTrait, CharacterTraitKind, registries::get_trait_from_registry_by_name,
            },
        },
        get_bvp_by_bv_name, normalise_bvp_name,
    },
    error::{SSCCResult, new_character_error, new_internal_error},
    modules::character::{
        name::CharacterName,
        skill::CharacterSkill,
        r#trait::CharacterTraitStore,
        utils::{
            new_random_char_birthday, new_random_char_iq, new_random_char_look,
            new_random_char_skills,
        },
    },
    utils::{
        Utils,
        boxes::{
            DataBoxSlotId, TeamBoxSlotId,
            data_box::{DataBox, flag_box::FlagBox, stat_box::StatBox},
        },
        op::Op,
    },
};

pub mod name;
pub mod skill;
pub mod r#trait;
pub mod utils;

pub struct Character {
    pub name: CharacterName,
    /// The character's data box
    data_box: DataBoxSlotId,
    pub birthday: Utc,
    // TODO: Retirement System with national retirement age, company benefits etc. At some point -
    // Fixed date should work for a good while => 70th Birthday
    // (Context: Global average in 2026 seems to be 60-65; tendency rising. (Denmark hits 70 in 2040))
    pub retirement: Utc,
    pub teams: Vec<TeamBoxSlotId>,
    /// The character's traits.
    /// All traits must be unique.
    traits: CharacterTraitStore,
    skills: Vec<CharacterSkill>,
}
impl Character {
    pub fn new_random(
        utils: &mut Utils,
        character_data_box_id: DataBoxSlotId,
        character_data_box: &mut DataBox,
        utc_now: &Utc,
    ) -> SSCCResult<Character> {
        let birthday = new_random_char_birthday(utils, utc_now)?;
        let looks = new_random_char_look(utils)?;
        let iq = new_random_char_iq(utils)?;
        character_data_box.stats.add_base_value(IQ.0, &iq.0);
        let traits = CharacterTraitStore::new_random(utils, iq.1, looks)?;
        let name = CharacterName::new_random(utils)?;
        let teams = Vec::new();
        let age_in_years =
            HpFloat::cast((*utc_now - birthday).as_secs()) / 60.0 / 60.0 / 24.0 / 365.2425;
        let mut char =
            Character::new_custom(character_data_box_id, traits, vec![], teams, birthday, name);
        for skill in new_random_char_skills(utils, age_in_years)? {
            char.add_skill(skill, character_data_box, utc_now)?;
        }
        Ok(char)
    }
    /// Generates a new random character with one random 'ultra' level skill.
    pub fn new_random_one_ultra_skill(
        utils: &mut Utils,
        character_data_box_id: DataBoxSlotId,
        character_data_box: &mut DataBox,
        utc_now: &Utc,
    ) -> SSCCResult<Character> {
        let mut character =
            Character::new_random(utils, character_data_box_id, character_data_box, utc_now)?;
        let base = &SKILL_REGISTRY_OF_REGISTRIES[utils
            .rng
            .random_index(SKILL_REGISTRY_OF_REGISTRIES.len())
            .map_err(|err| err.to_nemesis_error())?];
        let skill = base[utils
            .rng
            .random_index(base.len())
            .map_err(|err| err.to_nemesis_error())?];
        character.add_skill(skill, character_data_box, utc_now)?;

        let idx = character
            .skills
            .iter_mut()
            .position(|entry| entry.skill.name == skill.name)
            .expect("Skill was just added");
        let skill_mut_ptr = character.skills.get_mut(idx).expect("Skill was just added");

        let new_level = {
            utils
                .rng
                .random_from_range(0, SKILL_LEVEL_ULTRA_AMOUNT)
                .map_err(|err| err.to_nemesis_error())? as u8
        };
        skill_mut_ptr.set_level(Level::from_ordinal(new_level));
        Ok(character)
    }
    pub fn new_custom(
        character_data_box_id: DataBoxSlotId,
        traits: CharacterTraitStore,
        skills: Vec<CharacterSkill>,
        teams: Vec<TeamBoxSlotId>,
        birthday: Utc,
        name: CharacterName,
    ) -> Character {
        Character {
            name,
            data_box: character_data_box_id,
            retirement: birthday + Duration::from_hours((RETIREMENT_AGE as u64) * 365 * 24),
            birthday,
            teams,
            traits,
            skills,
        }
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
            let mut tmp_accum: HashMap<&str, Vec<(HpFloat, Op)>> = HashMap::new();
            for skill in self.skills.iter() {
                for impact in skill.skill.all_impacts() {
                    if stats.contains_key(impact.base_value_name) {
                        let impact_factor = impact
                            .calc_final_stat_impact(skill.affinity(), skill.effective_level());
                        if let Some(entry) = tmp_accum.get_mut(impact.base_value_name) {
                            entry.push(impact_factor);
                        } else {
                            tmp_accum.insert(impact.base_value_name, vec![impact_factor]);
                        }
                    }
                }
            }
            for (name, impacts) in tmp_accum.iter() {
                if let Some(stat) = stats.get_mut(name)
                    && let Some(stat) = stat.as_hp_float_mut()
                {
                    let mut limit = None;
                    let mut set = None;
                    let mut accum = HpFloat::new_zero();
                    for (factor, op) in impacts {
                        match op {
                            Op::Add => accum = accum + *factor,
                            Op::Sub => accum = accum - *factor,
                            Op::Mul => accum = accum * *factor,
                            Op::Div => accum = accum / *factor,
                            Op::Set => {
                                if let Some(set_value) = set
                                    && factor > set_value
                                {
                                    set = Some(factor);
                                }
                            }
                            Op::Lim => {
                                if let Some(lim_value) = limit
                                    && factor > lim_value
                                {
                                    limit = Some(factor);
                                }
                            }
                        }
                    }
                    if let Some(limit) = limit
                        && accum > *limit
                    {
                        accum = *limit;
                    }
                    if let Some(set) = set {
                        accum = *set;
                    }
                    *stat = *stat + accum;
                }
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
    // TODO: Adds `(POINT_COST_LOW_LEVEL) / 10.0` as point value base
    // Consider if a passed in base would be preferable
    pub fn tick_skill_use(
        &mut self,
        data_box_slot_map: &mut SlotMap<DataBox>,
        used_base_values: Option<&[&'static str]>,
        used_skills: Option<&[&'static str]>,
        utc_now: &Utc,
    ) -> SSCCResult<()> {
        let character_data_box: &mut DataBox = data_box_slot_map
            .get_mut(self.data_box)
            .ok_or(new_internal_error("Cannot get DataBox of Character!"))?;
        let points = HpFloat::cast(POINT_COST_LOW_LEVEL) / 10.0;
        if let Some(used_skills) = used_skills {
            for used_skill in used_skills {
                if let Some(idx) = self
                    .skills
                    .iter()
                    .position(|entry| entry.skill.name == *used_skill)
                    && let Some(used_skill) = self.skills.get_mut(idx)
                {
                    // Skill exists and needs advancement
                    used_skill.add_points(points);
                } else {
                    // Skill doesnt exist and needs to be added
                    if let Some(needed_skill) = get_skill_from_registry_by_name(used_skill) {
                        self.add_skill(needed_skill, character_data_box, utc_now)?;
                    };
                }
            }
        }
        if let Some(used_bvs) = used_base_values {
            let used_bvs = used_bvs
                .iter()
                .map(|entry| normalise_bvp_name(entry))
                .flatten();
            for used_bv in used_bvs {
                if let Some(idx) = self.skills.iter().position(|entry| {
                    if let Some(master) = entry.skill.master
                        && master.base_value_name == used_bv
                    {
                        true
                    } else {
                        false
                    }
                }) && let Some(used_skill) = self.skills.get_mut(idx)
                {
                    // Skill present and was used
                    used_skill.add_points(points);
                } else {
                    // No skill present / Or no master skill to push
                    if let Some(needed_skill) = get_master_skill_for_base_value(used_bv) {
                        self.add_skill(needed_skill, character_data_box, utc_now)?;
                    };
                }
            }
        }
        Ok(())
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
            self.skills
                .iter_mut()
                .for_each(|entry| entry.update_from_traits(&self.traits.get_all_traits()));
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
        for &conf in character_trait.conflicting_traits() {
            self.remove_trait(
                get_trait_from_registry_by_name(conf).expect("Make this safe"),
                data_box_slot_map,
            );
        }
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
    pub fn add_skill(
        &mut self,
        skill: &'static ConstSkill,
        character_data_box: &mut DataBox,
        utc_now: &Utc,
    ) -> SSCCResult<()> {
        let mut skill = CharacterSkill::new(skill, &self.traits.get_all_traits());
        let age_in_years =
            HpFloat::cast((*utc_now - self.birthday).as_secs()) / 60.0 / 60.0 / 24.0 / 365.2425;
        if let Some(iq) = character_data_box.stats.get_base_value(IQ.0)
            && let Some(iq) = iq.as_hp_float()
        {
            skill.normalise_char_skill_from_char(*iq, age_in_years);
        };
        for impact in skill.skill.all_impacts() {
            for bv_name in normalise_bvp_name(impact.base_value_name) {
                if !character_data_box.stats.is_tracking(bv_name) {
                    if let Some(bv_value) = get_bvp_by_bv_name(bv_name).1 {
                        character_data_box
                            .stats
                            .add_base_value(bv_name, &bv_value.into());
                    }
                }
            }
        }
        Ok(())
    }
}
