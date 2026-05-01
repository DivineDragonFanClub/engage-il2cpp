
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/physics/Physics.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Physics")]
#[parent(crate::system::object::Object)]
pub struct Physics {}

#[cfg(feature = "unity_engine-physics")]
#[::unity2::methods]
impl Physics {
    #[method(name = "get_gravity", args = 0)]
    pub fn get_gravity() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_defaultPhysicsScene", args = 0)]
    pub fn get_default_physics_scene() -> crate::unity_engine::physicsscene::PhysicsScene;

    #[method(name = "IgnoreCollision", args = 3)]
    pub fn ignore_collision(
        collider1: crate::unity_engine::collider::Collider,
        collider2: crate::unity_engine::collider::Collider,
        ignore: bool,
    ) -> ();

    #[method(name = "IgnoreCollision", args = 2)]
    pub fn ignore_collision_2(
        collider1: crate::unity_engine::collider::Collider,
        collider2: crate::unity_engine::collider::Collider,
    ) -> ();

    #[method(name = "Raycast", args = 5)]
    pub fn raycast(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_2(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_3(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
    ) -> bool;

    #[method(name = "Raycast", args = 2)]
    pub fn raycast_4(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "Raycast", args = 6)]
    pub fn raycast_5(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_6(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_7(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
    ) -> bool;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_8(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
    ) -> bool;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_9(
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_10(
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Raycast", args = 2)]
    pub fn raycast_11(ray: crate::unity_engine::ray::Ray, max_distance: f32) -> bool;

    #[method(name = "Raycast", args = 1)]
    pub fn raycast_12(ray: crate::unity_engine::ray::Ray) -> bool;

    #[method(name = "Raycast", args = 5)]
    pub fn raycast_13(
        ray: crate::unity_engine::ray::Ray,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Raycast", args = 4)]
    pub fn raycast_14(
        ray: crate::unity_engine::ray::Ray,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_15(
        ray: crate::unity_engine::ray::Ray,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
    ) -> bool;

    #[method(name = "Raycast", args = 2)]
    pub fn raycast_16(
        ray: crate::unity_engine::ray::Ray,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
    ) -> bool;

    #[method(name = "Linecast", args = 4)]
    pub fn linecast(
        start: crate::unity_engine::vector3::Vector3,
        end: crate::unity_engine::vector3::Vector3,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Linecast", args = 3)]
    pub fn linecast_2(
        start: crate::unity_engine::vector3::Vector3,
        end: crate::unity_engine::vector3::Vector3,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Linecast", args = 5)]
    pub fn linecast_3(
        start: crate::unity_engine::vector3::Vector3,
        end: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "Linecast", args = 4)]
    pub fn linecast_4(
        start: crate::unity_engine::vector3::Vector3,
        end: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "CapsuleCast", args = 8)]
    pub fn capsule_cast(
        point1: crate::unity_engine::vector3::Vector3,
        point2: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "SphereCast", args = 7)]
    pub fn sphere_cast(
        origin: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "SphereCast", args = 6)]
    pub fn sphere_cast_2(
        origin: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "SphereCast", args = 6)]
    pub fn sphere_cast_3(
        ray: crate::unity_engine::ray::Ray,
        radius: f32,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "SphereCast", args = 5)]
    pub fn sphere_cast_4(
        ray: crate::unity_engine::ray::Ray,
        radius: f32,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "BoxCast", args = 8)]
    pub fn box_cast(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        orientation: crate::unity_engine::quaternion::Quaternion,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "BoxCast", args = 7)]
    pub fn box_cast_2(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        orientation: crate::unity_engine::quaternion::Quaternion,
        max_distance: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "Internal_RaycastAll", args = 5)]
    pub fn internal_raycast_all(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 5)]
    pub fn raycast_all(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 4)]
    pub fn raycast_all_2(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 3)]
    pub fn raycast_all_3(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 2)]
    pub fn raycast_all_4(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 4)]
    pub fn raycast_all_5(
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 3)]
    pub fn raycast_all_6(
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 2)]
    pub fn raycast_all_7(
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastAll", args = 1)]
    pub fn raycast_all_8(
        ray: crate::unity_engine::ray::Ray,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "RaycastNonAlloc", args = 5)]
    pub fn raycast_non_alloc(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 4)]
    pub fn raycast_non_alloc_2(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 3)]
    pub fn raycast_non_alloc_3(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 2)]
    pub fn raycast_non_alloc_4(
        ray: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 6)]
    pub fn raycast_non_alloc_5(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 5)]
    pub fn raycast_non_alloc_6(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 4)]
    pub fn raycast_non_alloc_7(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
    ) -> i32;

    #[method(name = "RaycastNonAlloc", args = 3)]
    pub fn raycast_non_alloc_8(
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    ) -> i32;

    #[method(name = "OverlapSphere_Internal", args = 5)]
    pub fn overlap_sphere_internal(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::collider::Collider>;

    #[method(name = "OverlapSphere", args = 4)]
    pub fn overlap_sphere(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::collider::Collider>;

    #[method(name = "OverlapSphere", args = 3)]
    pub fn overlap_sphere_2(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
    ) -> ::unity2::Array<crate::unity_engine::collider::Collider>;

    #[method(name = "Simulate_Internal", args = 2)]
    pub fn simulate_internal(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        step: f32,
    ) -> ();

    #[method(name = "Simulate", args = 1)]
    pub fn simulate(step: f32) -> ();

    #[method(name = "get_autoSimulation", args = 0)]
    pub fn get_auto_simulation() -> bool;

    #[method(name = "set_autoSimulation", args = 1)]
    pub fn set_auto_simulation(value: bool) -> ();

    #[method(name = "Query_ComputePenetration", args = 8)]
    pub fn query_compute_penetration(
        collider_a: crate::unity_engine::collider::Collider,
        position_a: crate::unity_engine::vector3::Vector3,
        rotation_a: crate::unity_engine::quaternion::Quaternion,
        collider_b: crate::unity_engine::collider::Collider,
        position_b: crate::unity_engine::vector3::Vector3,
        rotation_b: crate::unity_engine::quaternion::Quaternion,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> bool;

    #[method(name = "ComputePenetration", args = 8)]
    pub fn compute_penetration(
        collider_a: crate::unity_engine::collider::Collider,
        position_a: crate::unity_engine::vector3::Vector3,
        rotation_a: crate::unity_engine::quaternion::Quaternion,
        collider_b: crate::unity_engine::collider::Collider,
        position_b: crate::unity_engine::vector3::Vector3,
        rotation_b: crate::unity_engine::quaternion::Quaternion,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> bool;

    #[method(name = "OverlapSphereNonAlloc", args = 5)]
    pub fn overlap_sphere_non_alloc(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        results: ::unity2::Array<crate::unity_engine::collider::Collider>,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "OverlapSphereNonAlloc", args = 4)]
    pub fn overlap_sphere_non_alloc_2(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        results: ::unity2::Array<crate::unity_engine::collider::Collider>,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "CheckSphere_Internal", args = 5)]
    pub fn check_sphere_internal(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "CheckSphere", args = 4)]
    pub fn check_sphere(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "CheckSphere", args = 3)]
    pub fn check_sphere_2(
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "SphereCastNonAlloc", args = 7)]
    pub fn sphere_cast_non_alloc(
        origin: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "SphereCastNonAlloc", args = 6)]
    pub fn sphere_cast_non_alloc_2(
        origin: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        max_distance: f32,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "CheckBox_Internal", args = 6)]
    pub fn check_box_internal(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        orientation: crate::unity_engine::quaternion::Quaternion,
        layermask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "CheckBox", args = 5)]
    pub fn check_box(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        orientation: crate::unity_engine::quaternion::Quaternion,
        layermask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "CheckBox", args = 4)]
    pub fn check_box_2(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        orientation: crate::unity_engine::quaternion::Quaternion,
        layer_mask: i32,
    ) -> bool;

    #[method(name = "OverlapBoxNonAlloc", args = 6)]
    pub fn overlap_box_non_alloc(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::collider::Collider>,
        orientation: crate::unity_engine::quaternion::Quaternion,
        mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "OverlapBoxNonAlloc", args = 5)]
    pub fn overlap_box_non_alloc_2(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::collider::Collider>,
        orientation: crate::unity_engine::quaternion::Quaternion,
        mask: i32,
    ) -> i32;

    #[method(name = "BoxCastNonAlloc", args = 8)]
    pub fn box_cast_non_alloc(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        orientation: crate::unity_engine::quaternion::Quaternion,
        max_distance: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> i32;

    #[method(name = "BoxCastNonAlloc", args = 7)]
    pub fn box_cast_non_alloc_2(
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        orientation: crate::unity_engine::quaternion::Quaternion,
        max_distance: f32,
        layer_mask: i32,
    ) -> i32;

    #[method(name = "get_gravity_Injected", args = 1)]
    pub fn get_gravity_injected(ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_defaultPhysicsScene_Injected", args = 1)]
    pub fn get_default_physics_scene_injected(
        ret: crate::unity_engine::physicsscene::PhysicsScene,
    ) -> ();

    #[method(name = "Internal_RaycastAll_Injected", args = 5)]
    pub fn internal_raycast_all_injected(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "OverlapSphere_Internal_Injected", args = 5)]
    pub fn overlap_sphere_internal_injected(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::collider::Collider>;

    #[method(name = "Simulate_Internal_Injected", args = 2)]
    pub fn simulate_internal_injected(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        step: f32,
    ) -> ();

    #[method(name = "Query_ComputePenetration_Injected", args = 8)]
    pub fn query_compute_penetration_injected(
        collider_a: crate::unity_engine::collider::Collider,
        position_a: crate::unity_engine::vector3::Vector3,
        rotation_a: crate::unity_engine::quaternion::Quaternion,
        collider_b: crate::unity_engine::collider::Collider,
        position_b: crate::unity_engine::vector3::Vector3,
        rotation_b: crate::unity_engine::quaternion::Quaternion,
        direction: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> bool;

    #[method(name = "CheckSphere_Internal_Injected", args = 5)]
    pub fn check_sphere_internal_injected(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        position: crate::unity_engine::vector3::Vector3,
        radius: f32,
        layer_mask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "CheckBox_Internal_Injected", args = 6)]
    pub fn check_box_internal_injected(
        physics_scene: crate::unity_engine::physicsscene::PhysicsScene,
        center: crate::unity_engine::vector3::Vector3,
        half_extents: crate::unity_engine::vector3::Vector3,
        orientation: crate::unity_engine::quaternion::Quaternion,
        layermask: i32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;
}
