
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/physics2d/Physics2D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Physics2D")]
#[parent(crate::system::object::Object)]
pub struct Physics2D {
    #[static_field]
    #[rename(name = "m_LastDisabledRigidbody2D")]
    pub m_last_disabled_rigidbody2_d: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rigidbody2d::Rigidbody2D,
    >,
}

#[cfg(feature = "unity_engine-physics2d")]
#[::unity2::methods]
impl Physics2D {
    #[method(name = "get_defaultPhysicsScene", args = 0)]
    pub fn get_default_physics_scene() -> crate::unity_engine::physicsscene2d::PhysicsScene2D;

    #[method(name = "get_queriesHitTriggers", args = 0)]
    pub fn get_queries_hit_triggers() -> bool;

    #[method(name = "Raycast", args = 2)]
    pub fn raycast(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_2(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_3(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_4(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        layer_mask: i32,
        min_depth: f32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 6)]
    pub fn raycast_5(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        distance: f32,
        layer_mask: i32,
        min_depth: f32,
        max_depth: f32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_6(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_7(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
        distance: f32,
    ) -> i32;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_8(
        origin: crate::unity_engine::vector2::Vector2,
        direction: crate::unity_engine::vector2::Vector2,
        contact_filter: crate::unity_engine::contactfilter2d::ContactFilter2D,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::raycasthit2d::RaycastHit2D,
        >,
        distance: f32,
    ) -> i32;

    #[method(name = "GetRayIntersectionAll", args = 1)]
    pub fn get_ray_intersection_all(
        ray: crate::unity_engine::ray::Ray,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;

    #[method(name = "GetRayIntersectionAll", args = 2)]
    pub fn get_ray_intersection_all_2(
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;

    #[method(name = "GetRayIntersectionAll", args = 3)]
    pub fn get_ray_intersection_all_3(
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;

    #[method(name = "GetRayIntersectionAll_Internal", args = 5)]
    pub fn get_ray_intersection_all_internal(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;

    #[method(name = "GetRayIntersectionNonAlloc", args = 2)]
    pub fn get_ray_intersection_non_alloc(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
    ) -> i32;

    #[method(name = "GetRayIntersectionNonAlloc", args = 3)]
    pub fn get_ray_intersection_non_alloc_2(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
        distance: f32,
    ) -> i32;

    #[method(name = "GetRayIntersectionNonAlloc", args = 4)]
    pub fn get_ray_intersection_non_alloc_3(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
        distance: f32,
        layer_mask: i32,
    ) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetRayIntersectionAll_Internal_Injected", args = 5)]
    pub fn get_ray_intersection_all_internal_injected(
        physics_scene: crate::unity_engine::physicsscene2d::PhysicsScene2D,
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;
}
