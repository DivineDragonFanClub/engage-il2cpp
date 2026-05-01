
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ai/navmeshpath/NavMeshPath.md")))]
#[::unity2::class(namespace = "UnityEngine.AI", name = "NavMeshPath")]
#[parent(crate::system::object::Object)]
pub struct NavMeshPath {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_Corners")]
    pub m_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "unity_engine-ai-navmeshpath")]
#[::unity2::methods]
impl NavMeshPath {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "InitializeNavMeshPath", args = 0)]
    pub fn initialize_nav_mesh_path() -> ::unity2::IntPtr;

    #[method(name = "DestroyNavMeshPath", args = 1)]
    pub fn destroy_nav_mesh_path(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "CalculateCornersInternal", args = 0)]
    pub fn calculate_corners_internal(
        self,
    ) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "ClearCornersInternal", args = 0)]
    pub fn clear_corners_internal(self) -> ();

    #[method(name = "ClearCorners", args = 0)]
    pub fn clear_corners(self) -> ();

    #[method(name = "CalculateCorners", args = 0)]
    pub fn calculate_corners(self) -> ();

    #[method(name = "get_corners", args = 0)]
    pub fn get_corners(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "get_status", args = 0)]
    pub fn get_status(self) -> crate::unity_engine::ai::navmeshpathstatus::NavMeshPathStatus;
}

#[cfg(feature = "unity_engine-ai-navmeshpath")]
impl NavMeshPath {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NavMeshPath),
                ::core::stringify!(new),
            )
        });
        <Self as INavMeshPathMethods>::ctor(this);
        this
    }
}
