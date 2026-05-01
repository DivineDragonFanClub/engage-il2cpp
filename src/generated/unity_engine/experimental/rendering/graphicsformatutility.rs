
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/graphicsformatutility/GraphicsFormatUtility.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering",
    name = "GraphicsFormatUtility"
)]
#[parent(crate::system::object::Object)]
pub struct GraphicsFormatUtility {}

#[cfg(feature = "unity_engine-experimental-rendering-graphicsformatutility")]
#[::unity2::methods]
impl GraphicsFormatUtility {
    #[method(name = "GetFormat", args = 1)]
    pub fn get_format(
        texture: crate::unity_engine::texture::Texture,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat", args = 2)]
    pub fn get_graphics_format(
        format: crate::unity_engine::textureformat::TextureFormat,
        is_srgb: bool,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat_Native_TextureFormat", args = 2)]
    pub fn get_graphics_format_native_texture_format(
        format: crate::unity_engine::textureformat::TextureFormat,
        is_srgb: bool,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat", args = 2)]
    pub fn get_graphics_format_2(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        is_srgb: bool,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat_Native_RenderTextureFormat", args = 2)]
    pub fn get_graphics_format_native_render_texture_format(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        is_srgb: bool,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat", args = 2)]
    pub fn get_graphics_format_3(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "IsSRGBFormat", args = 1)]
    pub fn is_srgb_format(
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> bool;

    #[method(name = "GetRenderTextureFormat", args = 1)]
    pub fn get_render_texture_format(
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> crate::unity_engine::rendertextureformat::RenderTextureFormat;

    #[method(name = "IsCompressedTextureFormat", args = 1)]
    pub fn is_compressed_texture_format(
        format: crate::unity_engine::textureformat::TextureFormat,
    ) -> bool;

    #[method(name = "IsCrunchFormat", args = 1)]
    pub fn is_crunch_format(format: crate::unity_engine::textureformat::TextureFormat) -> bool;
}
