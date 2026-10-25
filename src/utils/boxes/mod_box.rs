use std::collections::HashMap;

use aequa::{Object, XffSerde, XffValue, xff};
use nemesis::NemesisResultExt;

use crate::{error::new_xff_serde_error, modules::modifier::Modifier};

pub type ModifierName = String;

pub struct ModBox {
    modifiers: HashMap<String, Modifier>,
}

impl ModBox {
    pub fn new() -> ModBox {
        ModBox {
            modifiers: HashMap::new(),
        }
    }
    /// Add a modifier.
    ///
    /// Only adds the modifier if one of the same name doesn't already exist
    pub fn add_modifier(&mut self, mod_name: ModifierName, modifier: Modifier) {
        if !self.modifiers.contains_key(&mod_name) {
            self.modifiers.insert(mod_name, modifier);
        }
    }
    pub fn contains_modifier(&self, mod_name: &ModifierName) -> bool {
        self.modifiers.contains_key(mod_name)
    }
    pub fn get_modifier(&self, mod_name: &ModifierName) -> Option<Modifier> {
        self.modifiers.get(mod_name).cloned()
    }
}

impl XffSerde for ModBox {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        for (mn, m) in &self.modifiers {
            obj.insert(mn, m.to_xff());
        }
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            let mut out_mods = HashMap::new();
            for (key, value) in obj.iter() {
                out_mods.insert(key.clone(), Modifier::from_xff(value)?);
            }
            Ok(ModBox {
                modifiers: out_mods,
            })
        } else {
            Err(new_xff_serde_error(
                "Can't convert value to ModBox, not an object.",
            ))
            .add_ctx(format!("Value: {xff:?}"))
        }
    }
}
