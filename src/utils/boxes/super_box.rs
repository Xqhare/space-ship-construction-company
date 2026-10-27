use aequa::{Object, XffSerde, XffValue, xff};

use crate::{error::new_startup_error, runtime::DataBoxSlotId, utils::boxes::world_box::WorldBox};
#[non_exhaustive]
pub struct SuperBox {
    player_data_box_id: DataBoxSlotId,
    world_box: WorldBox,
}

impl XffSerde for SuperBox {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("player_store", self.player_data_box_id.to_xff());
        obj.insert("world_store", self.world_box.to_xff());
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, nemesis::NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(player_store) = obj.get("player_store")
            && let Some(world_store) = obj.get("world_store")
        {
            Ok(Self {
                player_data_box_id: DataBoxSlotId::from_xff(player_store)?,
                world_box: WorldBox::from_xff(world_store)?,
            })
        } else {
            Err(new_startup_error("Unable to load SuperStore from XFF"))
        }
    }
}

impl SuperBox {
    pub fn new<V: Into<Vec<char>>>(
        player_data_box_id: DataBoxSlotId,
        world_data_box_id: DataBoxSlotId,
        init_world_seed: V,
    ) -> SuperBox {
        SuperBox {
            player_data_box_id,
            world_box: WorldBox::new_seeded(world_data_box_id, init_world_seed.into()),
        }
    }
    /// Gives access to the player store
    pub fn player_data_box_id(&mut self) -> &DataBoxSlotId {
        &self.player_data_box_id
    }
    /// Gives access to the world store
    pub fn world_box(&mut self) -> &mut WorldBox {
        &mut self.world_box
    }
}
