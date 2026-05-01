
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/renderingutils/RenderingUtils_StereoConstants.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "RenderingUtils.StereoConstants"
)]
#[parent(crate::system::object::Object)]
pub struct RenderingUtils_StereoConstants {
    #[rename(name = "viewProjMatrix")]
    pub view_proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "invViewMatrix")]
    pub inv_view_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "invProjMatrix")]
    pub inv_proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "invViewProjMatrix")]
    pub inv_view_proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "invCameraProjMatrix")]
    pub inv_camera_proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "worldSpaceCameraPos")]
    pub world_space_camera_pos: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
}

#[cfg(feature = "unity_engine-rendering-universal-renderingutils")]
#[::unity2::methods]
impl RenderingUtils_StereoConstants {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-renderingutils")]
impl RenderingUtils_StereoConstants {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderingUtils_StereoConstants),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderingUtils_StereoConstantsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/renderingutils/RenderingUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "RenderingUtils")]
#[parent(crate::system::object::Object)]
pub struct RenderingUtils {
    #[static_field]
    #[rename(name = "m_LegacyShaderPassNames")]
    pub m_legacy_shader_pass_names: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::shadertagid::ShaderTagId,
    >,
    #[static_field]
    #[rename(name = "s_FullscreenMesh")]
    pub s_fullscreen_mesh: crate::unity_engine::mesh::Mesh,
    #[static_field]
    #[rename(name = "s_ErrorMaterial")]
    pub s_error_material: crate::unity_engine::material::Material,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_V")]
    pub unity_stereo_matrix_v: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_IV")]
    pub unity_stereo_matrix_iv: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_P")]
    pub unity_stereo_matrix_p: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_IP")]
    pub unity_stereo_matrix_ip: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_VP")]
    pub unity_stereo_matrix_vp: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_MATRIX_IVP")]
    pub unity_stereo_matrix_ivp: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_CAMERA_PROJECTION")]
    pub unity_stereo_camera_projection: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_CAMERA_INV_PROJECTION")]
    pub unity_stereo_camera_inv_projection: i32,
    #[static_field]
    #[rename(name = "UNITY_STEREO_VECTOR_CAMPOS")]
    pub unity_stereo_vector_campos: i32,
    #[static_field]
    #[rename(name = "stereoConstants")]
    pub stereo_constants:
        crate::unity_engine::rendering::universal::renderingutils::RenderingUtils_StereoConstants,
    #[static_field]
    #[rename(name = "m_RenderTextureFormatSupport")]
    pub m_render_texture_format_support:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::unity_engine::rendertextureformat::RenderTextureFormat,
            bool,
        >,
    #[static_field]
    #[rename(name = "m_GraphicsFormatSupport")]
    pub m_graphics_format_support: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::unity_engine::experimental::rendering::formatusage::FormatUsage,
            bool,
        >,
    >,
}

#[cfg(feature = "unity_engine-rendering-universal-renderingutils")]
#[::unity2::methods]
impl RenderingUtils {
    #[method(name = "get_fullscreenMesh", args = 0)]
    pub fn get_fullscreen_mesh() -> crate::unity_engine::mesh::Mesh;

    #[method(name = "get_useStructuredBuffer", args = 0)]
    pub fn get_use_structured_buffer() -> bool;

    #[method(name = "get_errorMaterial", args = 0)]
    pub fn get_error_material() -> crate::unity_engine::material::Material;

    #[method(name = "SetViewAndProjectionMatrices", args = 4)]
    pub fn set_view_and_projection_matrices(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        projection_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        set_inverse_matrices: bool,
    ) -> ();

    #[method(name = "SetStereoViewAndProjectionMatrices", args = 5)]
    pub fn set_stereo_view_and_projection_matrices(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        view_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        camera_proj_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        set_inverse_matrices: bool,
    ) -> ();

    #[method(name = "Blit", args = 10)]
    pub fn blit(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        destination: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        material: crate::unity_engine::material::Material,
        pass_index: i32,
        use_draw_procedural: bool,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "RenderObjectsWithError", args = 5)]
    pub fn render_objects_with_error(
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        camera: crate::unity_engine::camera::Camera,
        filter_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
        sort_flags: crate::unity_engine::rendering::sortingcriteria::SortingCriteria,
    ) -> ();

    #[method(name = "ClearSystemInfoCache", args = 0)]
    pub fn clear_system_info_cache() -> ();

    #[method(name = "SupportsRenderTextureFormat", args = 1)]
    pub fn supports_render_texture_format(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> bool;

    #[method(name = "SupportsGraphicsFormat", args = 2)]
    pub fn supports_graphics_format(
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        usage: crate::unity_engine::experimental::rendering::formatusage::FormatUsage,
    ) -> bool;

    #[method(name = "GetLastValidColorBufferIndex", args = 1)]
    pub fn get_last_valid_color_buffer_index(
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> i32;

    #[method(name = "GetValidColorBufferCount", args = 1)]
    pub fn get_valid_color_buffer_count(
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> u32;

    #[method(name = "IsMRT", args = 1)]
    pub fn is_mrt(
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> bool;

    #[method(name = "Contains", args = 2)]
    pub fn contains(
        source: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> bool;

    #[method(name = "IndexOf", args = 2)]
    pub fn index_of(
        source: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> i32;

    #[method(name = "CountDistinct", args = 2)]
    pub fn count_distinct(
        source: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> u32;

    #[method(name = "LastValid", args = 1)]
    pub fn last_valid(
        source: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> i32;

    #[method(name = "Contains", args = 2)]
    pub fn contains_2(
        a: crate::unity_engine::rendering::clearflag::ClearFlag,
        b: crate::unity_engine::rendering::clearflag::ClearFlag,
    ) -> bool;

    #[method(name = "SequenceEqual", args = 2)]
    pub fn sequence_equal(
        left: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        right: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
