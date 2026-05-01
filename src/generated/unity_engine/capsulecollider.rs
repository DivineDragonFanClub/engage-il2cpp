
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::collider::Collider;
use crate::unity_engine::collider::ICollider;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/capsulecollider/CapsuleCollider.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CapsuleCollider")]
#[parent(crate::unity_engine::collider::Collider)]
pub struct CapsuleCollider {}

#[cfg(feature = "unity_engine-capsulecollider")]
#[::unity2::methods]
impl CapsuleCollider {
    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_radius", args = 0)]
    pub fn get_radius(self) -> f32;

    #[method(name = "set_radius", args = 1)]
    pub fn set_radius(self, value: f32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: f32) -> ();

    #[method(name = "get_direction", args = 0)]
    pub fn get_direction(self) -> i32;

    #[method(name = "set_direction", args = 1)]
    pub fn set_direction(self, value: i32) -> ();

    #[method(name = "GetGlobalExtents", args = 0)]
    pub fn get_global_extents(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CalculateTransform", args = 0)]
    pub fn calculate_transform(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_center_Injected", args = 1)]
    pub fn get_center_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_center_Injected", args = 1)]
    pub fn set_center_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetGlobalExtents_Injected", args = 1)]
    pub fn get_global_extents_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "CalculateTransform_Injected", args = 1)]
    pub fn calculate_transform_injected(self, ret: crate::unity_engine::matrix4x4::Matrix4x4)
        -> ();
}

#[cfg(feature = "unity_engine-capsulecollider")]
impl CapsuleCollider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CapsuleCollider),
                ::core::stringify!(new),
            )
        });
        <Self as ICapsuleColliderMethods>::ctor(this);
        this
    }
}
