
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/nointerpminintparameter/NoInterpMinIntParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "NoInterpMinIntParameter")]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < i32 >)]
pub struct NoInterpMinIntParameter {
    #[rename(name = "min")]
    pub min: i32,
}

#[cfg(feature = "unity_engine-rendering-nointerpminintparameter")]
#[::unity2::methods]
impl NoInterpMinIntParameter {
    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, value: i32, min: i32, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-nointerpminintparameter")]
impl NoInterpMinIntParameter {
    pub fn new(value: i32, min: i32, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NoInterpMinIntParameter),
                ::core::stringify!(new),
            )
        });
        <Self as INoInterpMinIntParameterMethods>::ctor(this, value, min, override_state);
        this
    }
}
