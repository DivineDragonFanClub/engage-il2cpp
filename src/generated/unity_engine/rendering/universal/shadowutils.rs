
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/shadowutils/ShadowUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "ShadowUtils")]
#[parent(crate::system::object::Object)]
pub struct ShadowUtils {
    #[static_field]
    #[rename(name = "m_ShadowmapFormat")]
    pub m_shadowmap_format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    #[static_field]
    #[rename(name = "m_ForceShadowPointSampling")]
    pub m_force_shadow_point_sampling: bool,
}

#[cfg(feature = "unity_engine-rendering-universal-shadowutils")]
#[::unity2::methods]
impl ShadowUtils {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "ExtractDirectionalLightMatrix", args = 12)]
    pub fn extract_directional_light_matrix(
        cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
        shadow_light_index: i32,
        cascade_index: i32,
        shadowmap_width: i32,
        shadowmap_height: i32,
        shadow_resolution: i32,
        shadow_near_plane: f32,
        cascade_split_distance: crate::unity_engine::vector4::Vector4,
        shadow_slice_data : crate :: unity_engine :: rendering :: universal :: shadowslicedata :: ShadowSliceData,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> bool;

    #[method(name = "ExtractDirectionalLightMatrix_NoCascade", args = 11)]
    pub fn extract_directional_light_matrix_no_cascade(
        cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
        shadow_light_index: i32,
        shadowmap_width: i32,
        shadowmap_height: i32,
        shadow_resolution: i32,
        shadow_near_plane: f32,
        cascade_split_distance: crate::unity_engine::vector4::Vector4,
        shadow_slice_data : crate :: unity_engine :: rendering :: universal :: shadowslicedata :: ShadowSliceData,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> bool;

    #[method(name = "ExtractSpotLightMatrix", args = 6)]
    pub fn extract_spot_light_matrix(
        cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
        shadow_light_index: i32,
        shadow_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        proj_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> bool;

    #[method(name = "RenderShadowSlice", args = 6)]
    pub fn render_shadow_slice(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        shadow_slice_data : crate :: unity_engine :: rendering :: universal :: shadowslicedata :: ShadowSliceData,
        settings: crate::unity_engine::rendering::shadowdrawingsettings::ShadowDrawingSettings,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "RenderShadowSlice", args = 4)]
    pub fn render_shadow_slice_2(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        shadow_slice_data : crate :: unity_engine :: rendering :: universal :: shadowslicedata :: ShadowSliceData,
        settings: crate::unity_engine::rendering::shadowdrawingsettings::ShadowDrawingSettings,
    ) -> ();

    #[method(name = "GetMaxTileResolutionInAtlas", args = 3)]
    pub fn get_max_tile_resolution_in_atlas(
        atlas_width: i32,
        atlas_height: i32,
        tile_count: i32,
    ) -> i32;

    #[method(name = "ApplySliceTransform", args = 3)]
    pub fn apply_slice_transform(
        shadow_slice_data : crate :: unity_engine :: rendering :: universal :: shadowslicedata :: ShadowSliceData,
        atlas_width: i32,
        atlas_height: i32,
    ) -> ();

    #[method(name = "GetShadowBias", args = 5)]
    pub fn get_shadow_bias(
        shadow_light: crate::unity_engine::rendering::visiblelight::VisibleLight,
        shadow_light_index: i32,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
        light_projection_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        shadow_resolution: f32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "SetupShadowCasterConstantBuffer", args = 3)]
    pub fn setup_shadow_caster_constant_buffer(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        shadow_light: crate::unity_engine::rendering::visiblelight::VisibleLight,
        shadow_bias: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "GetTemporaryShadowTexture", args = 3)]
    pub fn get_temporary_shadow_texture(
        width: i32,
        height: i32,
        bits: i32,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetShadowTransform", args = 2)]
    pub fn get_shadow_transform(
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;
}
