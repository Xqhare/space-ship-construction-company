use std::{path::Path, rc::Rc};

use aequa::{Array, Object, XffValue, xff};
use fortuna::Fortuna;
use ilmarinen::WordSmith;
use nemesis::NemesisResultExt;
use thoth::Thoth;

use crate::{
    error::{SSCCError, SSCCResult, new_startup_error},
    modules::super_store::data_store::DataStore,
    utils::{seed_giver, seed_normaliser},
};

pub mod data_store;

#[non_exhaustive]
pub struct SuperStore {
    player_store: DataStore,
    world_store: DataStore,
    unicode: Rc<Thoth>,
    rng: Fortuna,
    word_smith: WordSmith,
    world_seed: Vec<u8>,
    world_seed_normalised: Vec<u8>,
}

impl SuperStore {
    pub fn new<P: AsRef<Path>>(word_smith_path: P) -> SSCCResult<SuperStore> {
        Self::new_seeded(
            seed_giver()?.iter().map(|c| *c as u8).collect::<Vec<u8>>(),
            word_smith_path,
        )
    }
    pub fn new_seeded<V: Into<Vec<u8>>, P: AsRef<Path>>(
        seed: V,
        word_smith_path: P,
    ) -> SSCCResult<SuperStore> {
        let world_seed = seed.into();
        let world_seed_normalised = seed_normaliser(world_seed.clone());
        let player_store = DataStore::new();
        let world_store = DataStore::new();
        let unicode =
            Rc::new(Thoth::new().add_ctx("Startup error during Unicode (Thoth) initialization")?);
        let rng = Fortuna::new(world_seed_normalised.clone());
        let word_smith = WordSmith::new_managed(word_smith_path.as_ref(), unicode.clone())
            .map_err(|err| err.to_nemesis_error())
            .add_ctx("Startup error during word smith (Ilmarinen) initialization")?;
        Ok(SuperStore {
            player_store,
            world_store,
            unicode,
            rng,
            word_smith,
            world_seed,
            world_seed_normalised,
        })
    }
    pub fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("player_store", self.player_store.to_xff());
        obj.insert("world_store", self.world_store.to_xff());
        obj.insert("rng", self.rng.to_xff());
        let seed = {
            let mut ary = Array::new();
            for s in self.world_seed.iter() {
                ary.push(*s);
            }
            ary
        };
        obj.insert("world_seed", seed);
        let seed_normal = {
            let mut ary = Array::new();
            for s in self.world_seed_normalised.iter() {
                ary.push(*s);
            }
            ary
        };
        obj.insert("world_seed_normalised", seed_normal);
        xff!(obj)
    }
    pub fn from_xff<P: AsRef<Path>>(xff: &XffValue, word_smith_path: P) -> SSCCResult<SuperStore> {
        if let Some(obj) = xff.as_object()
            && let Some(player_store) = obj.get("player_store")
            && let Some(world_store) = obj.get("world_store")
            && let Some(rng) = obj.get("rng")
            && let Some(world_seed) = obj.get("world_seed")
            && let Some(world_seed) = world_seed.as_array()
            && let Some(world_seed_normalised) = obj.get("world_seed_normalised")
            && let Some(world_seed_normalised) = world_seed_normalised.as_array()
        {
            let player_store = DataStore::from_xff(player_store)
                .add_ctx("Unable to load player store from XFF")?;
            let world_store =
                DataStore::from_xff(world_store).add_ctx("Unable to load world store from XFF")?;
            let rng = Fortuna::from_xff(rng)
                .map_err(|e| SSCCError::Startup(format!("{:?}", e)))
                .add_ctx("Unable to load RNG from XFF")?;
            let world_seed = {
                let mut out = Vec::with_capacity(world_seed.len());
                for v in world_seed.iter() {
                    if let Some(v) = v.as_number()
                        && let Some(v) = v.into_usize()
                    {
                        out.push(v as u8);
                    } else {
                        return Err(new_startup_error(
                            "Unable to load world seed from XFF. Not a number, usize, or valid u8.",
                        ));
                    }
                }
                out
            };
            let world_seed_normalised = {
                let mut out = Vec::with_capacity(world_seed_normalised.len());
                for v in world_seed_normalised.iter() {
                    if let Some(v) = v.as_number()
                        && let Some(v) = v.into_usize()
                    {
                        out.push(v as u8);
                    } else {
                        return Err(new_startup_error(
                            "Unable to load world seed from XFF. Not a number, usize, or valid u8.",
                        ));
                    }
                }
                out
            };
            let unicode = Rc::new(Thoth::new().add_ctx(
                "Unable to initalise Unicode (Thoth) during xff loading of SuperStore.",
            )?);
            Ok(SuperStore {
                player_store,
                world_store,
                unicode: Rc::clone(&unicode),
                rng,
                word_smith: WordSmith::new_managed(
                    Path::new(word_smith_path.as_ref()),
                    Rc::clone(&unicode),
                )
                .map_err(|err| err.to_nemesis_error())
                .add_ctx(
                    "Unable to initalise word smith (Ilmarinen) during xff loading of SuperStore.",
                )?,
                world_seed,
                world_seed_normalised,
            })
        } else {
            Err(new_startup_error("Unable to load SuperStore from XFF"))
        }
    }
    /// Gives access to the player store
    pub fn player_store(&mut self) -> &mut DataStore {
        &mut self.player_store
    }
    /// Gives access to the world store
    pub fn world_store(&mut self) -> &mut DataStore {
        &mut self.world_store
    }
    /// Gives access to the Unicode segmentation handler.
    pub fn unicode(&self) -> &Rc<Thoth> {
        &self.unicode
    }
    /// Gives access to the random number generator.
    pub fn rng(&mut self) -> &mut Fortuna {
        &mut self.rng
    }
    /// Gives access to the 'word smith' name generator.
    ///
    /// # Note
    /// Calling `mint_managed` on the return value will generate a new name.
    /// Calling `mint` on the return value will return an error because of no internally held RNG.
    pub fn word_smith(&mut self) -> &mut WordSmith {
        &mut self.word_smith
    }
    /// Gives read access to the world seed
    pub fn seed(&self) -> &Vec<u8> {
        &self.world_seed
    }
}
