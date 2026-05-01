
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/nointerpclampedintparameter/NoInterpClampedIntParameter.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering",
    name = "NoInterpClampedIntParameter"
)]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < i32 >)]
pub struct NoInterpClampedIntParameter {
    #[rename(name = "min")]
    pub min: i32,
    #[rename(name = "max")]
    pub max: i32,
}

#[cfg(feature = "unity_engine-rendering-nointerpclampedintparameter")]
#[::unity2::methods]
impl NoInterpClampedIntParameter {
    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, value: i32, min: i32, max: i32, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-nointerpclampedintparameter")]
impl NoInterpClampedIntParameter {
    pub fn new(value: i32, min: i32, max: i32, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NoInterpClampedIntParameter),
                ::core::stringify!(new),
            )
        });
        <Self as INoInterpClampedIntParameterMethods>::ctor(this, value, min, max, override_state);
        this
    }
}
