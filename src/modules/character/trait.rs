use athena::float::HpFloat;

use crate::{
    consts::characters::traits::{CharacterTrait, CharacterTraitKind},
    needs_home::accum_char_traits,
};

pub struct CharacterTraitStore {
    pub intellect: &'static CharacterTrait,
    pub looks: &'static CharacterTrait,
    pub health: Option<&'static CharacterTrait>,
    pub mental_health: Option<&'static CharacterTrait>,
    pub social: Option<&'static CharacterTrait>,
    pub work_ethic: Option<&'static CharacterTrait>,
    pub normal_traits: Vec<&'static CharacterTrait>,
}

impl CharacterTraitStore {
    pub fn remove_trait(&mut self, character_trait: &'static CharacterTrait) {
        if self.has_trait(character_trait.name) {
            match character_trait.kind {
                CharacterTraitKind::Intellect | CharacterTraitKind::Looks => {}
                CharacterTraitKind::Health => self.health = None,
                CharacterTraitKind::MentalHealth => self.mental_health = None,
                CharacterTraitKind::Social => self.social = None,
                CharacterTraitKind::WorkEthics => self.work_ethic = None,
                CharacterTraitKind::Normal => {
                    self.normal_traits
                        .retain(|existing| *existing != character_trait);
                }
            }
        }
    }
    pub fn make_all_traits_accum(&self) -> Vec<(&'static str, (HpFloat, HpFloat))> {
        accum_char_traits(&self.get_all_traits())
    }
    pub fn get_all_traits(&self) -> Vec<&'static CharacterTrait> {
        // Technically I need to reserve at most 6 more (CharTraitStore field amount - 1 )
        let mut out = Vec::with_capacity(self.normal_traits.len() + 10);
        out.extend(self.normal_traits.clone());
        out.push(self.intellect);
        out.push(self.looks);
        if let Some(r#trait) = self.health {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.mental_health {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.social {
            out.push(r#trait);
        }
        if let Some(r#trait) = self.work_ethic {
            out.push(r#trait);
        }
        out
    }
    pub fn get_trait(&self, character_trait_name: &'static str) -> Option<&'static CharacterTrait> {
        if character_trait_name == self.intellect.name {
            Some(self.intellect)
        } else if character_trait_name == self.looks.name {
            Some(self.looks)
        } else if let Some(r#trait) = self.health
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.mental_health
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.social
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else if let Some(r#trait) = self.work_ethic
            && r#trait.name == character_trait_name
        {
            Some(r#trait)
        } else {
            for r#trait in &self.normal_traits {
                if r#trait.name == character_trait_name {
                    return Some(r#trait);
                }
            }
            None
        }
    }
    pub fn has_trait(&self, character_trait_name: &'static str) -> bool {
        self.get_trait(character_trait_name).is_some()
    }
    /// Returns `true` if the trait was added.
    pub fn add_trait(&mut self, character_trait: &'static CharacterTrait) -> bool {
        match character_trait.kind {
            CharacterTraitKind::Looks | CharacterTraitKind::Intellect => false,
            CharacterTraitKind::Normal => {
                let has_duplicate_or_conflict = self.normal_traits.iter().any(|existing| {
                    *existing == character_trait
                        || character_trait
                            .conflicting_traits()
                            .contains(&existing.name)
                });
                if !has_duplicate_or_conflict {
                    self.normal_traits.push(character_trait);
                    true
                } else {
                    false
                }
            }
            _ => {
                let store = match character_trait.kind {
                    CharacterTraitKind::Looks
                    | CharacterTraitKind::Intellect
                    | CharacterTraitKind::Normal => {
                        unreachable!(
                            "CharacterTraitStore::add_trait, Match arm should be unreachable!"
                        )
                    }
                    CharacterTraitKind::Health => &mut self.health,
                    CharacterTraitKind::MentalHealth => &mut self.mental_health,
                    CharacterTraitKind::Social => &mut self.social,
                    CharacterTraitKind::WorkEthics => &mut self.work_ethic,
                };
                if store.is_none() {
                    *store = Some(character_trait);
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn force_trait(&mut self, character_trait: &'static CharacterTrait) {
        match character_trait.kind {
            CharacterTraitKind::Intellect => self.intellect = character_trait,
            CharacterTraitKind::Looks => self.looks = character_trait,
            CharacterTraitKind::Health => self.health = Some(character_trait),
            CharacterTraitKind::MentalHealth => self.mental_health = Some(character_trait),
            CharacterTraitKind::Social => self.social = Some(character_trait),
            CharacterTraitKind::WorkEthics => self.work_ethic = Some(character_trait),
            CharacterTraitKind::Normal => {
                self.normal_traits.retain(|existing| {
                    *existing != character_trait
                        && !character_trait
                            .conflicting_traits()
                            .contains(&existing.name)
                });
                self.normal_traits.push(character_trait);
            }
        }
    }
}
