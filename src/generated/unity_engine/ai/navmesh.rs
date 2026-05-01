
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ai/navmesh/NavMesh.md")))]
#[::unity2::class(namespace = "UnityEngine.AI", name = "NavMesh")]
#[parent(crate::system::object::Object)]
pub struct NavMesh {
    #[static_field]
    #[rename(name = "onPreUpdate")]
    pub on_pre_update: crate::unity_engine::ai::navmesh::NavMesh_OnNavMeshPreUpdate,
}

#[cfg(feature = "unity_engine-ai-navmesh")]
#[::unity2::methods]
impl NavMesh {
    #[method(name = "Internal_CallOnNavMeshPreUpdate", args = 0)]
    pub fn internal_call_on_nav_mesh_pre_update() -> ();

    #[method(name = "CalculatePath", args = 4)]
    pub fn calculate_path(
        source_position: crate::unity_engine::vector3::Vector3,
        target_position: crate::unity_engine::vector3::Vector3,
        area_mask: i32,
        path: crate::unity_engine::ai::navmeshpath::NavMeshPath,
    ) -> bool;

    #[method(name = "CalculatePathInternal", args = 4)]
    pub fn calculate_path_internal(
        source_position: crate::unity_engine::vector3::Vector3,
        target_position: crate::unity_engine::vector3::Vector3,
        area_mask: i32,
        path: crate::unity_engine::ai::navmeshpath::NavMeshPath,
    ) -> bool;

    #[method(name = "SamplePosition", args = 4)]
    pub fn sample_position(
        source_position: crate::unity_engine::vector3::Vector3,
        hit: crate::unity_engine::ai::navmeshhit::NavMeshHit,
        max_distance: f32,
        area_mask: i32,
    ) -> bool;

    #[method(name = "CalculatePathInternal_Injected", args = 4)]
    pub fn calculate_path_internal_injected(
        source_position: crate::unity_engine::vector3::Vector3,
        target_position: crate::unity_engine::vector3::Vector3,
        area_mask: i32,
        path: crate::unity_engine::ai::navmeshpath::NavMeshPath,
    ) -> bool;

    #[method(name = "SamplePosition_Injected", args = 4)]
    pub fn sample_position_injected(
        source_position: crate::unity_engine::vector3::Vector3,
        hit: crate::unity_engine::ai::navmeshhit::NavMeshHit,
        max_distance: f32,
        area_mask: i32,
    ) -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ai/navmesh/NavMesh_OnNavMeshPreUpdate.md")))]
#[::unity2::class(namespace = "UnityEngine.AI", name = "NavMesh.OnNavMeshPreUpdate")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct NavMesh_OnNavMeshPreUpdate {}

#[cfg(feature = "unity_engine-ai-navmesh")]
#[::unity2::methods]
impl NavMesh_OnNavMeshPreUpdate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-ai-navmesh")]
impl NavMesh_OnNavMeshPreUpdate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NavMesh_OnNavMeshPreUpdate),
                ::core::stringify!(new),
            )
        });
        <Self as INavMesh_OnNavMeshPreUpdateMethods>::ctor(this, object, method);
        this
    }
}
