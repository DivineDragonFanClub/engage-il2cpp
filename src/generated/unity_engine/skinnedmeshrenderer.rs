
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::renderer::IRenderer;
use crate::unity_engine::renderer::Renderer;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/skinnedmeshrenderer/SkinnedMeshRenderer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SkinnedMeshRenderer")]
#[parent(crate::unity_engine::renderer::Renderer)]
pub struct SkinnedMeshRenderer {}

#[cfg(feature = "unity_engine-skinnedmeshrenderer")]
#[::unity2::methods]
impl SkinnedMeshRenderer {
    #[method(name = "get_quality", args = 0)]
    pub fn get_quality(self) -> crate::unity_engine::skinquality::SkinQuality;

    #[method(name = "set_quality", args = 1)]
    pub fn set_quality(self, value: crate::unity_engine::skinquality::SkinQuality) -> ();

    #[method(name = "get_updateWhenOffscreen", args = 0)]
    pub fn get_update_when_offscreen(self) -> bool;

    #[method(name = "set_updateWhenOffscreen", args = 1)]
    pub fn set_update_when_offscreen(self, value: bool) -> ();

    #[method(name = "get_forceMatrixRecalculationPerRender", args = 0)]
    pub fn get_force_matrix_recalculation_per_render(self) -> bool;

    #[method(name = "set_forceMatrixRecalculationPerRender", args = 1)]
    pub fn set_force_matrix_recalculation_per_render(self, value: bool) -> ();

    #[method(name = "get_rootBone", args = 0)]
    pub fn get_root_bone(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_rootBone", args = 1)]
    pub fn set_root_bone(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_bones", args = 0)]
    pub fn get_bones(self) -> ::unity2::Array<crate::unity_engine::transform::Transform>;

    #[method(name = "set_bones", args = 1)]
    pub fn set_bones(self, value: ::unity2::Array<crate::unity_engine::transform::Transform>)
        -> ();

    #[method(name = "get_sharedMesh", args = 0)]
    pub fn get_shared_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "set_sharedMesh", args = 1)]
    pub fn set_shared_mesh(self, value: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "get_skinnedMotionVectors", args = 0)]
    pub fn get_skinned_motion_vectors(self) -> bool;

    #[method(name = "set_skinnedMotionVectors", args = 1)]
    pub fn set_skinned_motion_vectors(self, value: bool) -> ();

    #[method(name = "GetBlendShapeWeight", args = 1)]
    pub fn get_blend_shape_weight(self, index: i32) -> f32;

    #[method(name = "SetBlendShapeWeight", args = 2)]
    pub fn set_blend_shape_weight(self, index: i32, value: f32) -> ();

    #[method(name = "BakeMesh", args = 1)]
    pub fn bake_mesh(self, mesh: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "BakeMesh", args = 2)]
    pub fn bake_mesh_2(self, mesh: crate::unity_engine::mesh::Mesh, use_scale: bool) -> ();

    #[method(name = "GetLocalAABB", args = 0)]
    pub fn get_local_aabb(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "SetLocalAABB", args = 1)]
    pub fn set_local_aabb(self, b: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "get_localBounds", args = 0)]
    pub fn get_local_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "set_localBounds", args = 1)]
    pub fn set_local_bounds(self, value: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetLocalAABB_Injected", args = 1)]
    pub fn get_local_aabb_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "SetLocalAABB_Injected", args = 1)]
    pub fn set_local_aabb_injected(self, b: crate::unity_engine::bounds::Bounds) -> ();
}

#[cfg(feature = "unity_engine-skinnedmeshrenderer")]
impl SkinnedMeshRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkinnedMeshRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as ISkinnedMeshRendererMethods>::ctor(this);
        this
    }
}
