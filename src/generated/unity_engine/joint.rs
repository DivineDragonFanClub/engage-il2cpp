
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/joint/Joint.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Joint")]
#[parent(crate::unity_engine::component::Component)]
pub struct Joint {}

#[cfg(feature = "unity_engine-joint")]
#[::unity2::methods]
impl Joint {
    #[method(name = "get_connectedBody", args = 0)]
    pub fn get_connected_body(self) -> crate::unity_engine::rigidbody::Rigidbody;

    #[method(name = "set_connectedAnchor", args = 1)]
    pub fn set_connected_anchor(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "set_connectedAnchor_Injected", args = 1)]
    pub fn set_connected_anchor_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg(feature = "unity_engine-joint")]
impl Joint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Joint),
                ::core::stringify!(new),
            )
        });
        <Self as IJointMethods>::ctor(this);
        this
    }
}
