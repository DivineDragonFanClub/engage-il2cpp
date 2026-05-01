
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::collider2d::Collider2D;
use crate::unity_engine::collider2d::ICollider2D;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/compositecollider2d/CompositeCollider2D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CompositeCollider2D")]
#[parent(crate::unity_engine::collider2d::Collider2D)]
pub struct CompositeCollider2D {}

#[cfg(feature = "unity_engine-compositecollider2d")]
#[::unity2::methods]
impl CompositeCollider2D {
    #[method(name = "get_pathCount", args = 0)]
    pub fn get_path_count(self) -> i32;

    #[method(name = "get_pointCount", args = 0)]
    pub fn get_point_count(self) -> i32;

    #[method(name = "GetPath", args = 2)]
    pub fn get_path(
        self,
        index: i32,
        points: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    ) -> i32;

    #[method(name = "GetPathArray_Internal", args = 2)]
    pub fn get_path_array_internal(
        self,
        index: i32,
        points: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    ) -> i32;
}
