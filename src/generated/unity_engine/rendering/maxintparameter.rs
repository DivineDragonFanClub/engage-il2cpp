
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::intparameter::IIntParameter;
use crate::unity_engine::rendering::intparameter::IntParameter;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/maxintparameter/MaxIntParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "MaxIntParameter")]
#[parent(crate::unity_engine::rendering::intparameter::IntParameter)]
pub struct MaxIntParameter {
    #[rename(name = "max")]
    pub max: i32,
}

#[cfg(feature = "unity_engine-rendering-maxintparameter")]
#[::unity2::methods]
impl MaxIntParameter {
    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, value: i32, max: i32, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-maxintparameter")]
impl MaxIntParameter {
    pub fn new(value: i32, max: i32, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaxIntParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IMaxIntParameterMethods>::ctor(this, value, max, override_state);
        this
    }
}
