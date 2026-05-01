
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::collider::Collider;
use crate::unity_engine::collider::ICollider;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/boxcollider/BoxCollider.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "BoxCollider")]
#[parent(crate::unity_engine::collider::Collider)]
pub struct BoxCollider {}

#[cfg(feature = "unity_engine-boxcollider")]
#[::unity2::methods]
impl BoxCollider {
    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_extents", args = 0)]
    pub fn get_extents(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_extents", args = 1)]
    pub fn set_extents(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_center_Injected", args = 1)]
    pub fn get_center_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_center_Injected", args = 1)]
    pub fn set_center_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_size_Injected", args = 1)]
    pub fn get_size_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_size_Injected", args = 1)]
    pub fn set_size_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg(feature = "unity_engine-boxcollider")]
impl BoxCollider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoxCollider),
                ::core::stringify!(new),
            )
        });
        <Self as IBoxColliderMethods>::ctor(this);
        this
    }
}
