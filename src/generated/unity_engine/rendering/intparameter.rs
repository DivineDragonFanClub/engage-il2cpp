
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/intparameter/IntParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "IntParameter")]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < i32 >)]
pub struct IntParameter {}

#[cfg(feature = "unity_engine-rendering-intparameter")]
#[::unity2::methods]
impl IntParameter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, value: i32, override_state: bool) -> ();

    #[method(name = "Interp", args = 3)]
    pub fn interp(self, from: i32, to: i32, t: f32) -> ();
}

#[cfg(feature = "unity_engine-rendering-intparameter")]
impl IntParameter {
    pub fn new(value: i32, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IntParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IIntParameterMethods>::ctor(this, value, override_state);
        this
    }
}
