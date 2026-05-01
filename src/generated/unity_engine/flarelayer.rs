
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/flarelayer/FlareLayer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "FlareLayer")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct FlareLayer {}

#[cfg(feature = "unity_engine-flarelayer")]
#[::unity2::methods]
impl FlareLayer {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-flarelayer")]
impl FlareLayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FlareLayer),
                ::core::stringify!(new),
            )
        });
        <Self as IFlareLayerMethods>::ctor(this);
        this
    }
}
