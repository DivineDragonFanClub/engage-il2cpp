
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::intparameter::IIntParameter;
use crate::unity_engine::rendering::intparameter::IntParameter;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/clampedintparameter/ClampedIntParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ClampedIntParameter")]
#[parent(crate::unity_engine::rendering::intparameter::IntParameter)]
pub struct ClampedIntParameter {
    #[rename(name = "min")]
    pub min: i32,
    #[rename(name = "max")]
    pub max: i32,
}

#[cfg(feature = "unity_engine-rendering-clampedintparameter")]
#[::unity2::methods]
impl ClampedIntParameter {
    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, value: i32, min: i32, max: i32, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-clampedintparameter")]
impl ClampedIntParameter {
    pub fn new(value: i32, min: i32, max: i32, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClampedIntParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IClampedIntParameterMethods>::ctor(this, value, min, max, override_state);
        this
    }
}
