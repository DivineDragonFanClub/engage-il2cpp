
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/boolparameter/BoolParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "BoolParameter")]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < bool >)]
pub struct BoolParameter {}

#[cfg(feature = "unity_engine-rendering-boolparameter")]
#[::unity2::methods]
impl BoolParameter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, value: bool, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-boolparameter")]
impl BoolParameter {
    pub fn new(value: bool, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoolParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IBoolParameterMethods>::ctor(this, value, override_state);
        this
    }
}
