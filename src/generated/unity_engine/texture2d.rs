
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::texture::ITexture;
use crate::unity_engine::texture::Texture;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/texture2d/Texture2D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Texture2D")]
#[parent(crate::unity_engine::texture::Texture)]
pub struct Texture2D {}

#[cfg(feature = "unity_engine-texture2d")]
#[::unity2::methods]
impl Texture2D {
    #[method(name = "get_format", args = 0)]
    pub fn get_format(self) -> crate::unity_engine::textureformat::TextureFormat;

    #[method(name = "get_whiteTexture", args = 0)]
    pub fn get_white_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_blackTexture", args = 0)]
    pub fn get_black_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_redTexture", args = 0)]
    pub fn get_red_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_grayTexture", args = 0)]
    pub fn get_gray_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_linearGrayTexture", args = 0)]
    pub fn get_linear_gray_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_normalTexture", args = 0)]
    pub fn get_normal_texture() -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "Compress", args = 1)]
    pub fn compress(self, high_quality: bool) -> ();

    #[method(name = "Internal_CreateImpl", args = 7)]
    pub fn internal_create_impl(
        mono: crate::unity_engine::texture2d::Texture2D,
        w: i32,
        h: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> bool;

    #[method(name = "Internal_Create", args = 7)]
    pub fn internal_create(
        mono: crate::unity_engine::texture2d::Texture2D,
        w: i32,
        h: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "get_isReadable", args = 0)]
    pub fn get_is_readable(self) -> bool;

    #[method(name = "get_vtOnly", args = 0)]
    pub fn get_vt_only(self) -> bool;

    #[method(name = "ApplyImpl", args = 2)]
    pub fn apply_impl(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "ResizeImpl", args = 2)]
    pub fn resize_impl(self, width: i32, height: i32) -> bool;

    #[method(name = "SetPixelImpl", args = 4)]
    pub fn set_pixel_impl(
        self,
        image: i32,
        x: i32,
        y: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "GetPixelImpl", args = 3)]
    pub fn get_pixel_impl(self, image: i32, x: i32, y: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetPixelBilinearImpl", args = 3)]
    pub fn get_pixel_bilinear_impl(
        self,
        image: i32,
        u: f32,
        v: f32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "ResizeWithFormatImpl", args = 4)]
    pub fn resize_with_format_impl(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        has_mip_map: bool,
    ) -> bool;

    #[method(name = "ReadPixelsImpl", args = 4)]
    pub fn read_pixels_impl(
        self,
        source: crate::unity_engine::rect::Rect,
        dest_x: i32,
        dest_y: i32,
        recalculate_mip_maps: bool,
    ) -> ();

    #[method(name = "SetPixelsImpl", args = 7)]
    pub fn set_pixels_impl(
        self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        pixel: ::unity2::Array<crate::unity_engine::color::Color>,
        miplevel: i32,
        frame: i32,
    ) -> ();

    #[method(name = "LoadRawTextureDataImpl", args = 2)]
    pub fn load_raw_texture_data_impl(self, data: ::unity2::IntPtr, size: i32) -> bool;

    #[method(name = "LoadRawTextureDataImplArray", args = 1)]
    pub fn load_raw_texture_data_impl_array(self, data: ::unity2::Array<u8>) -> bool;

    #[method(name = "SetPixelDataImplArray", args = 5)]
    pub fn set_pixel_data_impl_array(
        self,
        data: ::unity2::IlInstance,
        mip_level: i32,
        element_size: i32,
        data_array_size: i32,
        source_data_start_index: i32,
    ) -> bool;

    #[method(name = "SetPixelDataImpl", args = 5)]
    pub fn set_pixel_data_impl(
        self,
        data: ::unity2::IntPtr,
        mip_level: i32,
        element_size: i32,
        data_array_size: i32,
        source_data_start_index: i32,
    ) -> bool;

    #[method(name = "GetWritableImageData", args = 1)]
    pub fn get_writable_image_data(self, frame: i32) -> ::unity2::IntPtr;

    #[method(name = "GetRawImageDataSize", args = 0)]
    pub fn get_raw_image_data_size(self) -> i64;

    #[method(name = "GenerateAtlasImpl", args = 4)]
    pub fn generate_atlas_impl(
        sizes: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        padding: i32,
        atlas_size: i32,
        rect: ::unity2::Array<crate::unity_engine::rect::Rect>,
    ) -> ();

    #[method(name = "get_isPreProcessed", args = 0)]
    pub fn get_is_pre_processed(self) -> bool;

    #[method(name = "get_streamingMipmaps", args = 0)]
    pub fn get_streaming_mipmaps(self) -> bool;

    #[method(name = "get_streamingMipmapsPriority", args = 0)]
    pub fn get_streaming_mipmaps_priority(self) -> i32;

    #[method(name = "get_requestedMipmapLevel", args = 0)]
    pub fn get_requested_mipmap_level(self) -> i32;

    #[method(name = "set_requestedMipmapLevel", args = 1)]
    pub fn set_requested_mipmap_level(self, value: i32) -> ();

    #[method(name = "get_minimumMipmapLevel", args = 0)]
    pub fn get_minimum_mipmap_level(self) -> i32;

    #[method(name = "set_minimumMipmapLevel", args = 1)]
    pub fn set_minimum_mipmap_level(self, value: i32) -> ();

    #[method(name = "get_loadAllMips", args = 0)]
    pub fn get_load_all_mips(self) -> bool;

    #[method(name = "set_loadAllMips", args = 1)]
    pub fn set_load_all_mips(self, value: bool) -> ();

    #[method(name = "get_calculatedMipmapLevel", args = 0)]
    pub fn get_calculated_mipmap_level(self) -> i32;

    #[method(name = "get_desiredMipmapLevel", args = 0)]
    pub fn get_desired_mipmap_level(self) -> i32;

    #[method(name = "get_loadingMipmapLevel", args = 0)]
    pub fn get_loading_mipmap_level(self) -> i32;

    #[method(name = "get_loadedMipmapLevel", args = 0)]
    pub fn get_loaded_mipmap_level(self) -> i32;

    #[method(name = "ClearRequestedMipmapLevel", args = 0)]
    pub fn clear_requested_mipmap_level(self) -> ();

    #[method(name = "IsRequestedMipmapLevelLoaded", args = 0)]
    pub fn is_requested_mipmap_level_loaded(self) -> bool;

    #[method(name = "ClearMinimumMipmapLevel", args = 0)]
    pub fn clear_minimum_mipmap_level(self) -> ();

    #[method(name = "UpdateExternalTexture", args = 1)]
    pub fn update_external_texture(self, native_tex: ::unity2::IntPtr) -> ();

    #[method(name = "SetAllPixels32", args = 2)]
    pub fn set_all_pixels32(
        self,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetBlockOfPixels32", args = 6)]
    pub fn set_block_of_pixels32(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        miplevel: i32,
    ) -> ();

    #[method(name = "GetRawTextureData", args = 0)]
    pub fn get_raw_texture_data(self) -> ::unity2::Array<u8>;

    #[method(name = "GetPixels", args = 5)]
    pub fn get_pixels(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        miplevel: i32,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetPixels", args = 4)]
    pub fn get_pixels_2(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetPixels32", args = 1)]
    pub fn get_pixels32(
        self,
        miplevel: i32,
    ) -> ::unity2::Array<crate::unity_engine::color32::Color32>;

    #[method(name = "GetPixels32", args = 0)]
    pub fn get_pixels32_2(self) -> ::unity2::Array<crate::unity_engine::color32::Color32>;

    #[method(name = "PackTextures", args = 4)]
    pub fn pack_textures(
        self,
        textures: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
        padding: i32,
        maximum_atlas_size: i32,
        make_no_longer_readable: bool,
    ) -> ::unity2::Array<crate::unity_engine::rect::Rect>;

    #[method(name = "PackTextures", args = 3)]
    pub fn pack_textures_2(
        self,
        textures: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
        padding: i32,
        maximum_atlas_size: i32,
    ) -> ::unity2::Array<crate::unity_engine::rect::Rect>;

    #[method(name = "PackTextures", args = 2)]
    pub fn pack_textures_3(
        self,
        textures: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
        padding: i32,
    ) -> ::unity2::Array<crate::unity_engine::rect::Rect>;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_3(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_4(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        mip_count: i32,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_5(
        self,
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        linear: bool,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_6(
        self,
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        linear: bool,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_7(
        self,
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        linear: bool,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_8(
        self,
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_9(self, width: i32, height: i32) -> ();

    #[method(name = "CreateExternalTexture", args = 6)]
    pub fn create_external_texture(
        width: i32,
        height: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        linear: bool,
        native_tex: ::unity2::IntPtr,
    ) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "SetPixel", args = 3)]
    pub fn set_pixel(self, x: i32, y: i32, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetPixel", args = 4)]
    pub fn set_pixel_2(
        self,
        x: i32,
        y: i32,
        color: crate::unity_engine::color::Color,
        mip_level: i32,
    ) -> ();

    #[method(name = "SetPixels", args = 6)]
    pub fn set_pixels(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetPixels", args = 5)]
    pub fn set_pixels_2(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
    ) -> ();

    #[method(name = "SetPixels", args = 2)]
    pub fn set_pixels_3(
        self,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetPixels", args = 1)]
    pub fn set_pixels_4(self, colors: ::unity2::Array<crate::unity_engine::color::Color>) -> ();

    #[method(name = "GetPixel", args = 2)]
    pub fn get_pixel(self, x: i32, y: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetPixel", args = 3)]
    pub fn get_pixel_2(self, x: i32, y: i32, mip_level: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetPixelBilinear", args = 2)]
    pub fn get_pixel_bilinear(self, u: f32, v: f32) -> crate::unity_engine::color::Color;

    #[method(name = "GetPixelBilinear", args = 3)]
    pub fn get_pixel_bilinear_2(
        self,
        u: f32,
        v: f32,
        mip_level: i32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "LoadRawTextureData", args = 2)]
    pub fn load_raw_texture_data(self, data: ::unity2::IntPtr, size: i32) -> ();

    #[method(name = "LoadRawTextureData", args = 1)]
    pub fn load_raw_texture_data_2(self, data: ::unity2::Array<u8>) -> ();

    #[method(name = "Apply", args = 2)]
    pub fn apply(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "Apply", args = 1)]
    pub fn apply_2(self, update_mipmaps: bool) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply_3(self) -> ();

    #[method(name = "Resize", args = 2)]
    pub fn resize(self, width: i32, height: i32) -> bool;

    #[method(name = "Resize", args = 4)]
    pub fn resize_2(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        has_mip_map: bool,
    ) -> bool;

    #[method(name = "Resize", args = 4)]
    pub fn resize_3(
        self,
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        has_mip_map: bool,
    ) -> bool;

    #[method(name = "ReadPixels", args = 4)]
    pub fn read_pixels(
        self,
        source: crate::unity_engine::rect::Rect,
        dest_x: i32,
        dest_y: i32,
        recalculate_mip_maps: bool,
    ) -> ();

    #[method(name = "ReadPixels", args = 3)]
    pub fn read_pixels_2(
        self,
        source: crate::unity_engine::rect::Rect,
        dest_x: i32,
        dest_y: i32,
    ) -> ();

    #[method(name = "GenerateAtlas", args = 4)]
    pub fn generate_atlas(
        sizes: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        padding: i32,
        atlas_size: i32,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rect::Rect,
        >,
    ) -> bool;

    #[method(name = "SetPixels32", args = 2)]
    pub fn set_pixels32(
        self,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetPixels32", args = 1)]
    pub fn set_pixels32_2(
        self,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
    ) -> ();

    #[method(name = "SetPixels32", args = 6)]
    pub fn set_pixels32_3(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetPixels32", args = 5)]
    pub fn set_pixels32_4(
        self,
        x: i32,
        y: i32,
        block_width: i32,
        block_height: i32,
        colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
    ) -> ();

    #[method(name = "GetPixels", args = 1)]
    pub fn get_pixels_3(self, miplevel: i32) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetPixels", args = 0)]
    pub fn get_pixels_4(self) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "SetPixelImpl_Injected", args = 4)]
    pub fn set_pixel_impl_injected(
        self,
        image: i32,
        x: i32,
        y: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "GetPixelImpl_Injected", args = 4)]
    pub fn get_pixel_impl_injected(
        self,
        image: i32,
        x: i32,
        y: i32,
        ret: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "GetPixelBilinearImpl_Injected", args = 4)]
    pub fn get_pixel_bilinear_impl_injected(
        self,
        image: i32,
        u: f32,
        v: f32,
        ret: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "ReadPixelsImpl_Injected", args = 4)]
    pub fn read_pixels_impl_injected(
        self,
        source: crate::unity_engine::rect::Rect,
        dest_x: i32,
        dest_y: i32,
        recalculate_mip_maps: bool,
    ) -> ();
}

#[cfg(feature = "unity_engine-texture2d")]
impl Texture2D {
    pub fn new(
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new),
            )
        });
        <Self as ITexture2DMethods>::ctor(
            this, width, height, format, flags, mip_count, native_tex,
        );
        this
    }

    pub fn new_2(
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITexture2DMethods>::ctor_2(this, width, height, format, flags);
        this
    }

    pub fn new_3(
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITexture2DMethods>::ctor_3(this, width, height, format, flags);
        this
    }

    pub fn new_4(
        width: i32,
        height: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        mip_count: i32,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITexture2DMethods>::ctor_4(this, width, height, format, mip_count, flags);
        this
    }

    pub fn new_5(
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        linear: bool,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_5),
            )
        });
        <Self as ITexture2DMethods>::ctor_5(
            this,
            width,
            height,
            texture_format,
            mip_count,
            linear,
            native_tex,
        );
        this
    }

    pub fn new_6(
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        linear: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_6),
            )
        });
        <Self as ITexture2DMethods>::ctor_6(this, width, height, texture_format, mip_count, linear);
        this
    }

    pub fn new_7(
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        linear: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_7),
            )
        });
        <Self as ITexture2DMethods>::ctor_7(this, width, height, texture_format, mip_chain, linear);
        this
    }

    pub fn new_8(
        width: i32,
        height: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_8),
            )
        });
        <Self as ITexture2DMethods>::ctor_8(this, width, height, texture_format, mip_chain);
        this
    }

    pub fn new_9(width: i32, height: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture2D),
                ::core::stringify!(new_9),
            )
        });
        <Self as ITexture2DMethods>::ctor_9(this, width, height);
        this
    }
}
