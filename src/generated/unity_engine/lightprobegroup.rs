
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/lightprobegroup/LightProbeGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LightProbeGroup")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct LightProbeGroup {}

#[cfg(feature = "unity_engine-lightprobegroup")]
#[::unity2::methods]
impl LightProbeGroup {
    #[method(name = "get_probePositions", args = 0)]
    pub fn get_probe_positions(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-lightprobegroup")]
impl LightProbeGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LightProbeGroup),
                ::core::stringify!(new),
            )
        });
        <Self as ILightProbeGroupMethods>::ctor(this);
        this
    }
}
