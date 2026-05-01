
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/filmgrainlookupparameter/FilmGrainLookupParameter.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "FilmGrainLookupParameter"
)]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < crate :: unity_engine :: rendering :: universal :: filmgrainlookup :: FilmGrainLookup >)]
pub struct FilmGrainLookupParameter {}

#[cfg(feature = "unity_engine-rendering-universal-filmgrainlookupparameter")]
#[::unity2::methods]
impl FilmGrainLookupParameter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        value: crate::unity_engine::rendering::universal::filmgrainlookup::FilmGrainLookup,
        override_state: bool,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-filmgrainlookupparameter")]
impl FilmGrainLookupParameter {
    pub fn new(
        value: crate::unity_engine::rendering::universal::filmgrainlookup::FilmGrainLookup,
        override_state: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FilmGrainLookupParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IFilmGrainLookupParameterMethods>::ctor(this, value, override_state);
        this
    }
}
