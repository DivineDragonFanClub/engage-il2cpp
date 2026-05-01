
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/texture/Texture.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Texture")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Texture {
    #[static_field]
    #[rename(name = "GenerateAllMips")]
    pub generate_all_mips: i32,
}

#[cfg(feature = "unity_engine-texture")]
#[::unity2::methods]
impl Texture {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_mipmapCount", args = 0)]
    pub fn get_mipmap_count(self) -> i32;

    #[method(name = "get_graphicsFormat", args = 0)]
    pub fn get_graphics_format(
        self,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetDataWidth", args = 0)]
    pub fn get_data_width(self) -> i32;

    #[method(name = "GetDataHeight", args = 0)]
    pub fn get_data_height(self) -> i32;

    #[method(name = "GetDimension", args = 0)]
    pub fn get_dimension(
        self,
    ) -> crate::unity_engine::rendering::texturedimension::TextureDimension;

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> i32;

    #[method(name = "set_width", args = 1)]
    pub fn set_width(self, value: i32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> i32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: i32) -> ();

    #[method(name = "set_dimension", args = 1)]
    pub fn set_dimension(
        self,
        value: crate::unity_engine::rendering::texturedimension::TextureDimension,
    ) -> ();

    #[method(name = "get_isReadable", args = 0)]
    pub fn get_is_readable(self) -> bool;

    #[method(name = "get_wrapMode", args = 0)]
    pub fn get_wrap_mode(self) -> crate::unity_engine::texturewrapmode::TextureWrapMode;

    #[method(name = "set_wrapMode", args = 1)]
    pub fn set_wrap_mode(self, value: crate::unity_engine::texturewrapmode::TextureWrapMode) -> ();

    #[method(name = "set_filterMode", args = 1)]
    pub fn set_filter_mode(self, value: crate::unity_engine::filtermode::FilterMode) -> ();

    #[method(name = "set_anisoLevel", args = 1)]
    pub fn set_aniso_level(self, value: i32) -> ();

    #[method(name = "set_mipMapBias", args = 1)]
    pub fn set_mip_map_bias(self, value: f32) -> ();

    #[method(name = "get_texelSize", args = 0)]
    pub fn get_texel_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Internal_GetActiveTextureColorSpace", args = 0)]
    pub fn internal_get_active_texture_color_space(self) -> i32;

    #[method(name = "get_activeTextureColorSpace", args = 0)]
    pub fn get_active_texture_color_space(self) -> crate::unity_engine::colorspace::ColorSpace;

    #[method(name = "GetPixelDataSize", args = 2)]
    pub fn get_pixel_data_size(self, mip_level: i32, element: i32) -> i32;

    #[method(name = "GetPixelDataOffset", args = 2)]
    pub fn get_pixel_data_offset(self, mip_level: i32, element: i32) -> i32;

    #[method(name = "ValidateFormat", args = 1)]
    pub fn validate_format(self, format: crate::unity_engine::textureformat::TextureFormat)
        -> bool;

    #[method(name = "ValidateFormat", args = 2)]
    pub fn validate_format_2(
        self,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        usage: crate::unity_engine::experimental::rendering::formatusage::FormatUsage,
    ) -> bool;

    #[method(name = "CreateNonReadableException", args = 1)]
    pub fn create_non_readable_exception(
        self,
        t: crate::unity_engine::texture::Texture,
    ) -> crate::unity_engine::unityexception::UnityException;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "get_texelSize_Injected", args = 1)]
    pub fn get_texel_size_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();
}

#[cfg(feature = "unity_engine-texture")]
impl Texture {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture),
                ::core::stringify!(new),
            )
        });
        <Self as ITextureMethods>::ctor(this);
        this
    }
}
