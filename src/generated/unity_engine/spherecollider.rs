
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::collider::Collider;
use crate::unity_engine::collider::ICollider;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/spherecollider/SphereCollider.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SphereCollider")]
#[parent(crate::unity_engine::collider::Collider)]
pub struct SphereCollider {}

#[cfg(feature = "unity_engine-spherecollider")]
#[::unity2::methods]
impl SphereCollider {
    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_radius", args = 0)]
    pub fn get_radius(self) -> f32;

    #[method(name = "set_radius", args = 1)]
    pub fn set_radius(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_center_Injected", args = 1)]
    pub fn get_center_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_center_Injected", args = 1)]
    pub fn set_center_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg(feature = "unity_engine-spherecollider")]
impl SphereCollider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SphereCollider),
                ::core::stringify!(new),
            )
        });
        <Self as ISphereColliderMethods>::ctor(this);
        this
    }
}
