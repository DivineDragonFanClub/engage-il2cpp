
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/collider/Collider.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Collider")]
#[parent(crate::unity_engine::component::Component)]
pub struct Collider {}

#[cfg(feature = "unity_engine-collider")]
#[::unity2::methods]
impl Collider {
    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "get_attachedRigidbody", args = 0)]
    pub fn get_attached_rigidbody(self) -> crate::unity_engine::rigidbody::Rigidbody;

    #[method(name = "get_isTrigger", args = 0)]
    pub fn get_is_trigger(self) -> bool;

    #[method(name = "set_isTrigger", args = 1)]
    pub fn set_is_trigger(self, value: bool) -> ();

    #[method(name = "ClosestPoint", args = 1)]
    pub fn closest_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "get_sharedMaterial", args = 0)]
    pub fn get_shared_material(self) -> crate::unity_engine::physicmaterial::PhysicMaterial;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast(
        self,
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        has_hit: bool,
    ) -> crate::unity_engine::raycasthit::RaycastHit;

    #[method(name = "Raycast", args = 3)]
    pub fn raycast_2(
        self,
        ray: crate::unity_engine::ray::Ray,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
    ) -> bool;

    #[method(name = "Internal_ClosestPointOnBounds", args = 3)]
    pub fn internal_closest_point_on_bounds(
        self,
        point: crate::unity_engine::vector3::Vector3,
        out_pos: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> ();

    #[method(name = "ClosestPointOnBounds", args = 1)]
    pub fn closest_point_on_bounds(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ClosestPoint_Injected", args = 2)]
    pub fn closest_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_bounds_Injected", args = 1)]
    pub fn get_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "Raycast_Injected", args = 4)]
    pub fn raycast_injected(
        self,
        ray: crate::unity_engine::ray::Ray,
        max_distance: f32,
        has_hit: bool,
        ret: crate::unity_engine::raycasthit::RaycastHit,
    ) -> ();

    #[method(name = "Internal_ClosestPointOnBounds_Injected", args = 3)]
    pub fn internal_closest_point_on_bounds_injected(
        self,
        point: crate::unity_engine::vector3::Vector3,
        out_pos: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> ();
}

#[cfg(feature = "unity_engine-collider")]
impl Collider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Collider),
                ::core::stringify!(new),
            )
        });
        <Self as IColliderMethods>::ctor(this);
        this
    }
}
