
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/cullingresults/CullingResults.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CullingResults {}

impl ::unity2::ClassIdentity for CullingResults {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "CullingResults";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CullingResults {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-cullingresults")]
#[::unity2::methods(value)]
impl CullingResults {
    #[method(name = "GetLightIndexCount", args = 1)]
    pub fn get_light_index_count(culling_results_ptr: ::unity2::IntPtr) -> i32;

    #[method(name = "GetReflectionProbeIndexCount", args = 1)]
    pub fn get_reflection_probe_index_count(culling_results_ptr: ::unity2::IntPtr) -> i32;

    #[method(name = "FillLightAndReflectionProbeIndices", args = 2)]
    pub fn fill_light_and_reflection_probe_indices(
        culling_results_ptr: ::unity2::IntPtr,
        compute_buffer: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "GetLightIndexMapSize", args = 1)]
    pub fn get_light_index_map_size(culling_results_ptr: ::unity2::IntPtr) -> i32;

    #[method(name = "FillLightIndexMap", args = 3)]
    pub fn fill_light_index_map(
        culling_results_ptr: ::unity2::IntPtr,
        index_map_ptr: ::unity2::IntPtr,
        index_map_size: i32,
    ) -> ();

    #[method(name = "SetLightIndexMap", args = 3)]
    pub fn set_light_index_map(
        culling_results_ptr: ::unity2::IntPtr,
        index_map_ptr: ::unity2::IntPtr,
        index_map_size: i32,
    ) -> ();

    #[method(name = "GetShadowCasterBounds", args = 3)]
    pub fn get_shadow_caster_bounds(
        culling_results_ptr: ::unity2::IntPtr,
        light_index: i32,
        bounds: crate::unity_engine::bounds::Bounds,
    ) -> bool;

    #[method(name = "ComputeSpotShadowMatricesAndCullingPrimitives", args = 5)]
    pub fn compute_spot_shadow_matrices_and_culling_primitives(
        culling_results_ptr: ::unity2::IntPtr,
        active_light_index: i32,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;

    #[method(
        name = "ComputeDirectionalShadowMatricesAndCullingPrimitives",
        args = 10
    )]
    pub fn compute_directional_shadow_matrices_and_culling_primitives(
        culling_results_ptr: ::unity2::IntPtr,
        active_light_index: i32,
        split_index: i32,
        split_count: i32,
        split_ratio: crate::unity_engine::vector3::Vector3,
        shadow_resolution: i32,
        shadow_near_plane_offset: f32,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;

    #[method(name = "get_lightAndReflectionProbeIndexCount", args = 0)]
    pub fn get_light_and_reflection_probe_index_count(self) -> i32;

    #[method(name = "FillLightAndReflectionProbeIndices", args = 1)]
    pub fn fill_light_and_reflection_probe_indices_2(
        self,
        compute_buffer: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "GetShadowCasterBounds", args = 2)]
    pub fn get_shadow_caster_bounds_2(
        self,
        light_index: i32,
        out_bounds: crate::unity_engine::bounds::Bounds,
    ) -> bool;

    #[method(name = "ComputeSpotShadowMatricesAndCullingPrimitives", args = 4)]
    pub fn compute_spot_shadow_matrices_and_culling_primitives_2(
        self,
        active_light_index: i32,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;

    #[method(
        name = "ComputeDirectionalShadowMatricesAndCullingPrimitives",
        args = 9
    )]
    pub fn compute_directional_shadow_matrices_and_culling_primitives_2(
        self,
        active_light_index: i32,
        split_index: i32,
        split_count: i32,
        split_ratio: crate::unity_engine::vector3::Vector3,
        shadow_resolution: i32,
        shadow_near_plane_offset: f32,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::cullingresults::CullingResults,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(
        name = "ComputeDirectionalShadowMatricesAndCullingPrimitives_Injected",
        args = 10
    )]
    pub fn compute_directional_shadow_matrices_and_culling_primitives_injected(
        culling_results_ptr: ::unity2::IntPtr,
        active_light_index: i32,
        split_index: i32,
        split_count: i32,
        split_ratio: crate::unity_engine::vector3::Vector3,
        shadow_resolution: i32,
        shadow_near_plane_offset: f32,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;
}
