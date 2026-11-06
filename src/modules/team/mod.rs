use crate::utils::boxes::CharacterBoxSlotId;

pub struct Team {
    pub team_kind: TeamKind,
    pub name: String,
    pub members: Vec<CharacterBoxSlotId>,
}

pub enum TeamKind {
    Research,
    Development,
    Management,
}
