use std::time::Duration;

use aequa::{Object, XffSerde, XffValue, xff};
use fortuna::helpers::ascii_seed_normaliser;
use horae::Utc;
use nemesis::NemesisResultExt;

use crate::{
    error::{SSCCResult, new_xff_serde_error},
    runtime::DataBoxSlotId,
    utils::{seed_giver, start_date},
};

#[derive(Debug)]
#[non_exhaustive]
pub struct WorldBox {
    world_data_box_id: DataBoxSlotId,
    world_seed: Vec<u8>,
    world_seed_normalised: Vec<u8>,
    world_time: Utc,
}

impl XffSerde for WorldBox {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("world_data_box_id", self.world_data_box_id.to_xff());
        obj.insert("world_seed", self.world_seed.clone());
        obj.insert("world_seed_normalised", self.world_seed_normalised.clone());
        obj.insert("world_time", self.world_time.to_xff());
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(world_data_box) = obj.get("world_data_box_id")
            && let Some(world_seed) = obj.get("world_seed")
            && let Some(world_seed) = world_seed.as_array()
            && let Some(world_seed_normalised) = obj.get("world_seed_normalised")
            && let Some(world_seed_normalised) = world_seed_normalised.as_array()
            && let Some(world_time) = obj.get("world_time")
        {
            let world_time =
                Utc::from_xff(world_time).add_ctx("Can't create WorldBox, Invalid world_time.")?;
            let world_seed = {
                let mut out = Vec::with_capacity(world_seed.len());
                for byte in world_seed.iter() {
                    if let Some(byte) = byte.as_number()
                        && let Some(byte) = byte.into_usize()
                    {
                        out.push(byte as u8);
                    } else {
                        return Err(new_xff_serde_error(
                            "Can't create WorldBox, Invalid world_seed, value not a number.",
                        ))
                        .add_ctx(format!("Parent Value: {xff:?}"))
                        .add_ctx(format!("Value: {byte}"));
                    }
                }
                out
            };
            let world_seed_normalised = {
                let mut out = Vec::with_capacity(world_seed_normalised.len());
                for byte in world_seed_normalised.iter() {
                    if let Some(byte) = byte.as_number()
                        && let Some(byte) = byte.into_usize()
                    {
                        out.push(byte as u8);
                    } else {
                        return Err(new_xff_serde_error(
                            "Can't create WorldBox, Invalid world_seed_normalised, value not a number.",
                        ))
                        .add_ctx(format!("Parent Value: {xff:?}"))
                        .add_ctx(format!("Value: {byte}"));
                    }
                }
                out
            };
            Ok(WorldBox {
                world_data_box_id: DataBoxSlotId::from_xff(world_data_box)?,
                world_seed,
                world_seed_normalised,
                world_time,
            })
        } else {
            Err(new_xff_serde_error("Unable to load WorldBox from XFF"))
                .add_ctx(format!("Value: {xff:?}"))
        }
    }
}

impl WorldBox {
    pub fn new(world_data_box: DataBoxSlotId) -> SSCCResult<WorldBox> {
        let world_seed = seed_giver()?;
        Ok(Self::new_seeded(world_data_box, world_seed))
    }
    pub fn new_seeded(world_data_box_id: DataBoxSlotId, init_world_seed: Vec<char>) -> WorldBox {
        let world_seed = init_world_seed
            .iter()
            .map(|c| *c as u8)
            .collect::<Vec<u8>>();
        let world_seed_normalised = ascii_seed_normaliser(init_world_seed);
        let world_time = start_date();
        WorldBox {
            world_data_box_id,
            world_seed,
            world_seed_normalised,
            world_time,
        }
    }
    pub fn world_data_box_id(&self) -> DataBoxSlotId {
        self.world_data_box_id
    }
    pub fn world_seed(&self) -> &Vec<u8> {
        &self.world_seed
    }
    pub fn world_seed_normalised(&self) -> &Vec<u8> {
        &self.world_seed_normalised
    }
    pub fn now_utc(&self) -> Utc {
        self.world_time
    }
    pub fn world_time(&mut self) -> &mut Utc {
        &mut self.world_time
    }
    pub fn inc_world_time(&mut self, duration: Duration) {
        self.world_time = self.world_time + duration;
    }
}
