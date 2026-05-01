
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/nointerpvector3parameter/NoInterpVector3Parameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "NoInterpVector3Parameter")]
# [parent (crate :: unity_engine :: rendering :: volumeparameter_1 :: VolumeParameter_1 < crate :: unity_engine :: vector3 :: Vector3 >)]
pub struct NoInterpVector3Parameter {}

#[cfg(feature = "unity_engine-rendering-nointerpvector3parameter")]
#[::unity2::methods]
impl NoInterpVector3Parameter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, value: crate::unity_engine::vector3::Vector3, override_state: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-nointerpvector3parameter")]
impl NoInterpVector3Parameter {
    pub fn new(value: crate::unity_engine::vector3::Vector3, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NoInterpVector3Parameter),
                ::core::stringify!(new),
            )
        });
        <Self as INoInterpVector3ParameterMethods>::ctor(this, value, override_state);
        this
    }
}
