use crate::consts::characters::traits::CharacterTrait;

pub struct CharacterTraitStore {
    pub intellect: &'static CharacterTrait,
    pub looks: &'static CharacterTrait,
    pub health: Option<&'static CharacterTrait>,
    pub mental_health: Option<&'static CharacterTrait>,
    pub social: Option<&'static CharacterTrait>,
    pub work_ethic: Option<&'static CharacterTrait>,
    pub general_traits: Vec<&'static CharacterTrait>,
}
