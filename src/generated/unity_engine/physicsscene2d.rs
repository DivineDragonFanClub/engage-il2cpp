
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/physicsscene2d/PhysicsScene2D.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PhysicsScene2D {
    pub m_handle: i32,
}

impl ::unity2::ClassIdentity for PhysicsScene2D {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "PhysicsScene2D";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PhysicsScene2D {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-physicsscene2d")]
#[::unity2::methods(value)]
impl PhysicsScene2D {
    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::physicsscene2d::PhysicsScene2D) -> bool;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast(
        self,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_2(
        self,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast_Internal", args = 5)]
    pub fn raycast_internal(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_3(
        self,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "RaycastArray_Internal", args = 6)]
    pub fn raycast_array_internal(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_4(
        self,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::raycasthit2d::RaycastHit2D,
        >,
    ) -> i32;

    #[method(name = "RaycastList_Internal", args = 6)]
    pub fn raycast_list_internal(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::raycasthit2d::RaycastHit2D,
        >,
    ) -> i32;

    #[method(name = "GetRayIntersection", args = 4)]
    pub fn get_ray_intersection(
        self,
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "GetRayIntersectionArray_Internal", args = 6)]
    pub fn get_ray_intersection_array_internal(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
        layer_mask: i32,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "Raycast_Internal_Injected", args = 6)]
    pub fn raycast_internal_injected(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        ret: crate::unity_engine::raycasthit2d::RaycastHit2D,
    ) -> ();

    #[method(name = "RaycastArray_Internal_Injected", args = 6)]
    pub fn raycast_array_internal_injected(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "RaycastList_Internal_Injected", args = 6)]
    pub fn raycast_list_internal_injected(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::raycasthit2d::RaycastHit2D,
        >,
    ) -> i32;

    #[method(name = "GetRayIntersectionArray_Internal_Injected", args = 6)]
    pub fn get_ray_intersection_array_internal_injected(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
        layer_mask: i32,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;
}
