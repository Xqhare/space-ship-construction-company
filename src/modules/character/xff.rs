use aequa::{Array, Object, XffSerde, XffValue, xff};
use horae::Utc;

use crate::{
    error::new_xff_serde_error,
    modules::character::{
        Character, name::CharacterName, skill::CharacterSkill, r#trait::CharacterTraitStore,
    },
    utils::boxes::{DataBoxSlotId, TeamBoxSlotId},
};

const NAME_FIELD: &str = "name";
const DATA_BOX_FIELD: &str = "data_box";
const BIRTHDAY_FIELD: &str = "birthday";
const RETIREMENT_FIELD: &str = "retirement";
const TEAMS_FIELD: &str = "teams";
const TRAITS_FIELD: &str = "trais";
const SKILLS_FIELD: &str = "skills";

impl XffSerde for Character {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert(NAME_FIELD, self.name.to_xff());
        obj.insert(DATA_BOX_FIELD, self.data_box.to_xff());
        obj.insert(BIRTHDAY_FIELD, self.birthday.to_xff());
        obj.insert(RETIREMENT_FIELD, self.retirement.to_xff());
        let teams = {
            let mut ary = Array::new();
            for team in &self.teams {
                ary.push(team.to_xff());
            }
            ary
        };
        obj.insert(TEAMS_FIELD, teams);
        obj.insert(TRAITS_FIELD, self.traits.to_xff());
        let skills = {
            let mut ary = Array::new();
            for skill in &self.skills {
                ary.push(skill.to_xff());
            }
            ary
        };
        obj.insert(SKILLS_FIELD, skills);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(name) = obj.get(NAME_FIELD)
                && let Some(data_box) = obj.get(DATA_BOX_FIELD)
                && let Some(birthday) = obj.get(BIRTHDAY_FIELD)
                && let Some(retirement) = obj.get(RETIREMENT_FIELD)
                && let Some(teams) = obj.get(TEAMS_FIELD)
                && let Some(teams) = teams.as_array()
                && let Some(traits) = obj.get(TRAITS_FIELD)
                && let Some(skills) = obj.get(SKILLS_FIELD)
                && let Some(skills) = skills.as_array()
            {
                let name = CharacterName::from_xff(name)?;
                let data_box = DataBoxSlotId::from_xff(data_box)?;
                let birthday = Utc::from_xff(birthday)?;
                let retirement = Utc::from_xff(retirement)?;
                let teams = {
                    let mut tmp = Vec::with_capacity(teams.len());
                    for team in teams {
                        tmp.push(TeamBoxSlotId::from_xff(team)?);
                    }
                    tmp
                };
                let traits = CharacterTraitStore::from_xff(traits)?;
                let skills = {
                    let mut tmp = Vec::with_capacity(skills.len());
                    for skill in skills {
                        tmp.push(CharacterSkill::from_xff(skill)?);
                    }
                    tmp
                };
                Ok(Character {
                    name,
                    data_box,
                    birthday,
                    retirement,
                    teams,
                    traits,
                    skills,
                })
            } else {
                Err(
                    new_xff_serde_error("Cannot parse Character; Required fields missing!")
                        .add_ctx(format!("Value: '{xff:?}'")),
                )
            }
        } else {
            Err(
                new_xff_serde_error("Cannot parse Character; XffValue not an object!")
                    .add_ctx(format!("{xff:?}")),
            )
        }
    }
}
