
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/coreutils/CoreUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "CoreUtils")]
#[parent(crate::system::object::Object)]
pub struct CoreUtils {
    #[static_field]
    #[rename(name = "lookAtList")]
    pub look_at_list: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "upVectorList")]
    pub up_vector_list: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "editMenuPriority1")]
    pub edit_menu_priority1: i32,
    #[static_field]
    #[rename(name = "editMenuPriority2")]
    pub edit_menu_priority2: i32,
    #[static_field]
    #[rename(name = "editMenuPriority3")]
    pub edit_menu_priority3: i32,
    #[static_field]
    #[rename(name = "editMenuPriority4")]
    pub edit_menu_priority4: i32,
    #[static_field]
    #[rename(name = "assetCreateMenuPriority1")]
    pub asset_create_menu_priority1: i32,
    #[static_field]
    #[rename(name = "assetCreateMenuPriority2")]
    pub asset_create_menu_priority2: i32,
    #[static_field]
    #[rename(name = "assetCreateMenuPriority3")]
    pub asset_create_menu_priority3: i32,
    #[static_field]
    #[rename(name = "gameObjectMenuPriority")]
    pub game_object_menu_priority: i32,
    #[static_field]
    #[rename(name = "m_BlackCubeTexture")]
    pub m_black_cube_texture: crate::unity_engine::cubemap::Cubemap,
    #[static_field]
    #[rename(name = "m_MagentaCubeTexture")]
    pub m_magenta_cube_texture: crate::unity_engine::cubemap::Cubemap,
    #[static_field]
    #[rename(name = "m_MagentaCubeTextureArray")]
    pub m_magenta_cube_texture_array: crate::unity_engine::cubemaparray::CubemapArray,
    #[static_field]
    #[rename(name = "m_WhiteCubeTexture")]
    pub m_white_cube_texture: crate::unity_engine::cubemap::Cubemap,
    #[static_field]
    #[rename(name = "m_EmptyUAV")]
    pub m_empty_uav: crate::unity_engine::rendertexture::RenderTexture,
    #[static_field]
    #[rename(name = "m_BlackVolumeTexture")]
    pub m_black_volume_texture: crate::unity_engine::texture3d::Texture3D,
    #[static_field]
    #[rename(name = "m_AssemblyTypes")]
    pub m_assembly_types:
        crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>,
}

#[cfg(feature = "unity_engine-rendering-coreutils")]
#[::unity2::methods]
impl CoreUtils {
    #[method(name = "get_blackCubeTexture", args = 0)]
    pub fn get_black_cube_texture() -> crate::unity_engine::cubemap::Cubemap;

    #[method(name = "get_magentaCubeTexture", args = 0)]
    pub fn get_magenta_cube_texture() -> crate::unity_engine::cubemap::Cubemap;

    #[method(name = "get_magentaCubeTextureArray", args = 0)]
    pub fn get_magenta_cube_texture_array() -> crate::unity_engine::cubemaparray::CubemapArray;

    #[method(name = "get_whiteCubeTexture", args = 0)]
    pub fn get_white_cube_texture() -> crate::unity_engine::cubemap::Cubemap;

    #[method(name = "get_emptyUAV", args = 0)]
    pub fn get_empty_uav() -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "get_blackVolumeTexture", args = 0)]
    pub fn get_black_volume_texture() -> crate::unity_engine::texture3d::Texture3D;

    #[method(name = "ClearRenderTarget", args = 3)]
    pub fn clear_render_target(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "FixupDepthSlice", args = 2)]
    pub fn fixup_depth_slice(
        depth_slice: i32,
        buffer: crate::unity_engine::rendering::rthandle::RTHandle,
    ) -> i32;

    #[method(name = "FixupDepthSlice", args = 2)]
    pub fn fixup_depth_slice_2(
        depth_slice: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
    ) -> i32;

    #[method(name = "SetRenderTarget", args = 7)]
    pub fn set_render_target(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_2(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_3(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 7)]
    pub fn set_render_target_4(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 8)]
    pub fn set_render_target_5(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 3)]
    pub fn set_render_target_6(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 4)]
    pub fn set_render_target_7(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_8(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_9(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        load_action: crate::unity_engine::rendering::renderbufferloadaction::RenderBufferLoadAction,
        store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_10(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        load_action: crate::unity_engine::rendering::renderbufferloadaction::RenderBufferLoadAction,
        store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 9)]
    pub fn set_render_target_11(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 8)]
    pub fn set_render_target_12(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
    ) -> ();

    #[method(name = "SetViewportAndClear", args = 4)]
    pub fn set_viewport_and_clear(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 7)]
    pub fn set_render_target_13(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_14(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_15(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 7)]
    pub fn set_render_target_16(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 8)]
    pub fn set_render_target_17(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
        miplevel: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 3)]
    pub fn set_render_target_18(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 4)]
    pub fn set_render_target_19(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_20(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_buffer: crate::unity_engine::rendering::rthandle::RTHandle,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetViewport", args = 2)]
    pub fn set_viewport(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        target: crate::unity_engine::rendering::rthandle::RTHandle,
    ) -> ();

    #[method(name = "GetRenderTargetAutoName", args = 8)]
    pub fn get_render_target_auto_name(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        name: ::unity2::Il2CppString,
        mips: bool,
        enable_msaa: bool,
        msaa_samples: crate::unity_engine::rendering::msaasamples::MSAASamples,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetRenderTargetAutoName", args = 8)]
    pub fn get_render_target_auto_name_2(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        name: ::unity2::Il2CppString,
        mips: bool,
        enable_msaa: bool,
        msaa_samples: crate::unity_engine::rendering::msaasamples::MSAASamples,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetRenderTargetAutoName", args = 8)]
    pub fn get_render_target_auto_name_3(
        width: i32,
        height: i32,
        depth: i32,
        format: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        mips: bool,
        enable_msaa: bool,
        msaa_samples: crate::unity_engine::rendering::msaasamples::MSAASamples,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTextureAutoName", args = 7)]
    pub fn get_texture_auto_name(
        width: i32,
        height: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        dim: crate::unity_engine::rendering::texturedimension::TextureDimension,
        name: ::unity2::Il2CppString,
        mips: bool,
        depth: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTextureAutoName", args = 7)]
    pub fn get_texture_auto_name_2(
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        dim: crate::unity_engine::rendering::texturedimension::TextureDimension,
        name: ::unity2::Il2CppString,
        mips: bool,
        depth: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTextureAutoName", args = 7)]
    pub fn get_texture_auto_name_3(
        width: i32,
        height: i32,
        format: ::unity2::Il2CppString,
        dim: crate::unity_engine::rendering::texturedimension::TextureDimension,
        name: ::unity2::Il2CppString,
        mips: bool,
        depth: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ClearCubemap", args = 4)]
    pub fn clear_cubemap(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        clear_color: crate::unity_engine::color::Color,
        clear_mips: bool,
    ) -> ();

    #[method(name = "DrawFullScreen", args = 4)]
    pub fn draw_full_screen(
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        material: crate::unity_engine::material::Material,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
        shader_pass_id: i32,
    ) -> ();

    #[method(name = "DrawFullScreen", args = 5)]
    pub fn draw_full_screen_2(
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        material: crate::unity_engine::material::Material,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
        shader_pass_id: i32,
    ) -> ();

    #[method(name = "DrawFullScreen", args = 6)]
    pub fn draw_full_screen_3(
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        material: crate::unity_engine::material::Material,
        color_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_stencil_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
        shader_pass_id: i32,
    ) -> ();

    #[method(name = "DrawFullScreen", args = 6)]
    pub fn draw_full_screen_4(
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        material: crate::unity_engine::material::Material,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_stencil_buffer : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
        shader_pass_id: i32,
    ) -> ();

    #[method(name = "DrawFullScreen", args = 5)]
    pub fn draw_full_screen_5(
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        material: crate::unity_engine::material::Material,
        color_buffers: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
        shader_pass_id: i32,
    ) -> ();

    #[method(name = "ConvertSRGBToActiveColorSpace", args = 1)]
    pub fn convert_srgb_to_active_color_space(
        color: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "ConvertLinearToActiveColorSpace", args = 1)]
    pub fn convert_linear_to_active_color_space(
        color: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "CreateEngineMaterial", args = 1)]
    pub fn create_engine_material(
        shader_path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "CreateEngineMaterial", args = 1)]
    pub fn create_engine_material_2(
        shader: crate::unity_engine::shader::Shader,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "SetKeyword", args = 3)]
    pub fn set_keyword(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        keyword: ::unity2::Il2CppString,
        state: bool,
    ) -> ();

    #[method(name = "SetKeyword", args = 3)]
    pub fn set_keyword_2(
        material: crate::unity_engine::material::Material,
        keyword: ::unity2::Il2CppString,
        state: bool,
    ) -> ();

    #[method(name = "SetKeyword", args = 3)]
    pub fn set_keyword_3(
        cs: crate::unity_engine::computeshader::ComputeShader,
        keyword: ::unity2::Il2CppString,
        state: bool,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(obj: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "GetAllAssemblyTypes", args = 0)]
    pub fn get_all_assembly_types(
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>;

    #[method(name = "SafeRelease", args = 1)]
    pub fn safe_release(buffer: crate::unity_engine::computebuffer::ComputeBuffer) -> ();

    #[method(name = "CreateCubeMesh", args = 2)]
    pub fn create_cube_mesh(
        min: crate::unity_engine::vector3::Vector3,
        max: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "ArePostProcessesEnabled", args = 1)]
    pub fn are_post_processes_enabled(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "AreAnimatedMaterialsEnabled", args = 1)]
    pub fn are_animated_materials_enabled(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "IsSceneLightingDisabled", args = 1)]
    pub fn is_scene_lighting_disabled(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "IsLightOverlapDebugEnabled", args = 1)]
    pub fn is_light_overlap_debug_enabled(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "IsSceneViewFogEnabled", args = 1)]
    pub fn is_scene_view_fog_enabled(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "DrawRendererList", args = 3)]
    pub fn draw_renderer_list(
        render_context : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        renderer_list: crate::unity_engine::experimental::rendering::rendererlist::RendererList,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
