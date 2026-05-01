
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ai/navmeshdata/NavMeshData.md")))]
#[::unity2::class(namespace = "UnityEngine.AI", name = "NavMeshData")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct NavMeshData {}

#[cfg(feature = "unity_engine-ai-navmeshdata")]
#[::unity2::methods]
impl NavMeshData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, agent_type_id: i32) -> ();

    #[method(name = "Internal_Create", args = 2)]
    pub fn internal_create(
        mono: crate::unity_engine::ai::navmeshdata::NavMeshData,
        agent_type_id: i32,
    ) -> ();

    #[method(name = "get_sourceBounds", args = 0)]
    pub fn get_source_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_rotation", args = 1)]
    pub fn set_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_sourceBounds_Injected", args = 1)]
    pub fn get_source_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "get_position_Injected", args = 1)]
    pub fn get_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_position_Injected", args = 1)]
    pub fn set_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation_Injected", args = 1)]
    pub fn get_rotation_injected(self, ret: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "set_rotation_Injected", args = 1)]
    pub fn set_rotation_injected(self, value: crate::unity_engine::quaternion::Quaternion) -> ();
}

#[cfg(feature = "unity_engine-ai-navmeshdata")]
impl NavMeshData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NavMeshData),
                ::core::stringify!(new),
            )
        });
        <Self as INavMeshDataMethods>::ctor(this);
        this
    }

    pub fn new_2(agent_type_id: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NavMeshData),
                ::core::stringify!(new_2),
            )
        });
        <Self as INavMeshDataMethods>::ctor_2(this, agent_type_id);
        this
    }
}
