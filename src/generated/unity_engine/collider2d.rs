
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/collider2d/Collider2D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Collider2D")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Collider2D {}

#[cfg(feature = "unity_engine-collider2d")]
#[::unity2::methods]
impl Collider2D {
    #[method(name = "get_attachedRigidbody", args = 0)]
    pub fn get_attached_rigidbody(self) -> crate::unity_engine::rigidbody2d::Rigidbody2D;

    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "OverlapPoint", args = 1)]
    pub fn overlap_point(self, point: crate::unity_engine::vector2::Vector2) -> bool;

    #[method(name = "get_bounds_Injected", args = 1)]
    pub fn get_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "OverlapPoint_Injected", args = 1)]
    pub fn overlap_point_injected(self, point: crate::unity_engine::vector2::Vector2) -> bool;
}
