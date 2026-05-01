
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/graphics/Graphics.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Graphics")]
#[parent(crate::system::object::Object)]
pub struct Graphics {
    #[static_field]
    #[rename(name = "kMaxDrawMeshInstanceCount")]
    pub k_max_draw_mesh_instance_count: i32,
}

#[cfg(feature = "unity_engine-graphics")]
#[::unity2::methods]
impl Graphics {
    #[method(name = "Internal_GetMaxDrawMeshInstanceCount", args = 0)]
    pub fn internal_get_max_draw_mesh_instance_count() -> i32;

    #[method(name = "get_activeTier", args = 0)]
    pub fn get_active_tier() -> crate::unity_engine::rendering::graphicstier::GraphicsTier;

    #[method(name = "set_activeTier", args = 1)]
    pub fn set_active_tier(value: crate::unity_engine::rendering::graphicstier::GraphicsTier)
        -> ();

    #[method(name = "GetPreserveFramebufferAlpha", args = 0)]
    pub fn get_preserve_framebuffer_alpha() -> bool;

    #[method(name = "GetMinOpenGLESVersion", args = 0)]
    pub fn get_min_open_gles_version(
    ) -> crate::unity_engine::rendering::openglesversion::OpenGLESVersion;

    #[method(name = "CopyTexture_Slice", args = 6)]
    pub fn copy_texture_slice(
        src: crate::unity_engine::texture::Texture,
        src_element: i32,
        src_mip: i32,
        dst: crate::unity_engine::texture::Texture,
        dst_element: i32,
        dst_mip: i32,
    ) -> ();

    #[method(name = "Internal_DrawTexture", args = 1)]
    pub fn internal_draw_texture(
        args: crate::unity_engine::internal_drawtexturearguments::Internal_DrawTextureArguments,
    ) -> ();

    #[method(name = "CopyTexture", args = 6)]
    pub fn copy_texture(
        src: crate::unity_engine::texture::Texture,
        src_element: i32,
        src_mip: i32,
        dst: crate::unity_engine::texture::Texture,
        dst_element: i32,
        dst_mip: i32,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
