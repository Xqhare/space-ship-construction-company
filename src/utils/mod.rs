use std::{
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    rc::Rc,
};

use aequa::{Data, Object, XffSerde, XffValue, xff};
use fortuna::Fortuna;
use ilmarinen::Ilmarinen;
use nemesis::{NemesisError, NemesisResultExt};
use thoth::Thoth;

use crate::error::{SSCCResult, new_startup_error};

pub mod boxes;
mod functions;
pub use functions::*;

pub struct Utils {
    pub unicode: Rc<Thoth>,
    pub rng: Fortuna,
    pub word_smith: Ilmarinen,
    word_smith_path: PathBuf,
}

impl XffSerde for Utils {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("rng", self.rng.to_xff());
        obj.insert(
            "word_smith_path",
            Data::from(self.word_smith_path.as_os_str().as_bytes().to_vec()),
        );
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object()
            && let Some(rng) = obj.get("rng")
            && let Some(word_smith_path) = obj.get("word_smith_path")
            && let Some(word_smith_path) = word_smith_path.as_data()
        {
            let rng = Fortuna::from_xff(rng)
                .map_err(|err| err.to_nemesis_error())
                .add_ctx("Startup error during Unicode (Thoth) initialization")?;

            let word_smith_path = PathBuf::from(OsStr::from_bytes(word_smith_path.as_slice()));

            let unicode = Rc::new(
                Thoth::new().add_ctx("Startup error during Unicode (Thoth) initialization")?,
            );

            let word_smith = Ilmarinen::new_managed(word_smith_path.as_ref(), unicode.clone())
                .map_err(|err| err.to_nemesis_error())
                .add_ctx("Startup error during word smith (Ilmarinen) initialization")?;
            Ok(Self {
                unicode,
                rng,
                word_smith,
                word_smith_path,
            })
        } else {
            Err(new_startup_error("Unable to load Utils from XFF"))
        }
    }
}

impl Utils {
    pub fn new<P: AsRef<Path>>(
        word_smith_path: P,
        world_seed_normalised: Vec<u8>,
    ) -> SSCCResult<Utils> {
        let word_smith_path = word_smith_path.as_ref().to_path_buf();
        let unicode =
            Rc::new(Thoth::new().add_ctx("Startup error during Unicode (Thoth) initialization")?);
        let rng = Fortuna::new(world_seed_normalised);
        let word_smith = Ilmarinen::new_managed(word_smith_path.as_ref(), unicode.clone())
            .map_err(|err| err.to_nemesis_error())
            .add_ctx("Startup error during word smith (Ilmarinen) initialization")?;
        Ok(Utils {
            unicode,
            rng,
            word_smith,
            word_smith_path,
        })
    }
}
