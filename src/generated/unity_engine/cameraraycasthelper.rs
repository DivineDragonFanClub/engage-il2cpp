
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cameraraycasthelper/CameraRaycastHelper.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CameraRaycastHelper")]
#[parent(crate::system::object::Object)]
pub struct CameraRaycastHelper {}

#[cfg(feature = "unity_engine-cameraraycasthelper")]
#[::unity2::methods]
impl CameraRaycastHelper {
    #[method(name = "RaycastTry", args = 4)]
    pub fn raycast_try(
        cam: crate::unity_engine::camera::Camera,
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "RaycastTry2D", args = 4)]
    pub fn raycast_try2_d(
        cam: crate::unity_engine::camera::Camera,
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "RaycastTry_Injected", args = 4)]
    pub fn raycast_try_injected(
        cam: crate::unity_engine::camera::Camera,
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "RaycastTry2D_Injected", args = 4)]
    pub fn raycast_try2_d_injected(
        cam: crate::unity_engine::camera::Camera,
        ray: crate::unity_engine::ray::Ray,
        distance: f32,
        layer_mask: i32,
    ) -> crate::unity_engine::gameobject::GameObject;
}
