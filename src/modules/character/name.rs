use aequa::{Object, XffSerde, XffValue, xff};
use ilmarinen::{MintingObjectType, ObjectResult};

use crate::{
    error::{SSCCResult, new_xff_serde_error},
    utils::Utils,
};

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub struct CharacterName {
    pub legal_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub nick_name: String,
}

impl CharacterName {
    /// Generates a random character name
    pub fn new_random(utils: &mut Utils) -> SSCCResult<Self> {
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
    /// Creates a fully custom character name
    pub fn new_custom<S: ToString>(
        legal_name: S,
        first_name: S,
        middle_name: Option<S>,
        last_name: S,
        nick_name: S,
    ) -> Self {
        CharacterName {
            legal_name: legal_name.to_string(),
            first_name: first_name.to_string(),
            middle_name: middle_name.map(|s| s.to_string()),
            last_name: last_name.to_string(),
            nick_name: nick_name.to_string(),
        }
    }
}

const LEGAL_NAME_FIELD: &str = "legal_name";
const FIRST_NAME_FIELD: &str = "first_name";
const MIDDLE_NAME_FIELD: &str = "middle_name";
const LAST_NAME_FIELD: &str = "last_name";
const NICK_NAME_FIELD: &str = "nick_name";

impl XffSerde for CharacterName {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert(LEGAL_NAME_FIELD, &self.legal_name);
        obj.insert(FIRST_NAME_FIELD, &self.first_name);
        if let Some(middle) = &self.middle_name {
            obj.insert(MIDDLE_NAME_FIELD, middle);
        }
        obj.insert(LAST_NAME_FIELD, &self.last_name);
        obj.insert(NICK_NAME_FIELD, &self.nick_name);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(legal) = obj.get(LEGAL_NAME_FIELD)
                && let Some(first) = obj.get(FIRST_NAME_FIELD)
                && let Some(last) = obj.get(LAST_NAME_FIELD)
                && let Some(nick) = obj.get(NICK_NAME_FIELD)
            {
                let legal_name = legal.to_string();
                let first_name = first.to_string();
                let last_name = last.to_string();
                let nick_name = nick.to_string();
                let middle_name = {
                    if let Some(middle) = obj.get(MIDDLE_NAME_FIELD) {
                        Some(middle.to_string())
                    } else {
                        None
                    }
                };
                Ok(CharacterName {
                    legal_name,
                    first_name,
                    middle_name,
                    last_name,
                    nick_name,
                })
            } else {
                Err(
                    new_xff_serde_error("Cannot parse CharacterName; Required fields missing!")
                        .add_ctx(format!("Value: '{xff:?}'")),
                )
            }
        } else {
            Err(
                new_xff_serde_error("Cannot parse CharacterName; XffValue not an object!")
                    .add_ctx(format!("{xff:?}")),
            )
        }
    }
}
