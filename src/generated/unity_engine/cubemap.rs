
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::texture::ITexture;
use crate::unity_engine::texture::Texture;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cubemap/Cubemap.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Cubemap")]
#[parent(crate::unity_engine::texture::Texture)]
pub struct Cubemap {}

#[cfg(feature = "unity_engine-cubemap")]
#[::unity2::methods]
impl Cubemap {
    #[method(name = "get_format", args = 0)]
    pub fn get_format(self) -> crate::unity_engine::textureformat::TextureFormat;

    #[method(name = "Internal_CreateImpl", args = 6)]
    pub fn internal_create_impl(
        mono: crate::unity_engine::cubemap::Cubemap,
        ext: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> bool;

    #[method(name = "Internal_Create", args = 6)]
    pub fn internal_create(
        mono: crate::unity_engine::cubemap::Cubemap,
        ext: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "ApplyImpl", args = 2)]
    pub fn apply_impl(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "UpdateExternalTexture", args = 1)]
    pub fn update_external_texture(self, native_texture: ::unity2::IntPtr) -> ();

    #[method(name = "get_isReadable", args = 0)]
    pub fn get_is_readable(self) -> bool;

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

    #[method(name = "SmoothEdges", args = 1)]
    pub fn smooth_edges(self, smooth_region_width_in_pixels: i32) -> ();

    #[method(name = "SmoothEdges", args = 0)]
    pub fn smooth_edges_2(self) -> ();

    #[method(name = "GetPixels", args = 2)]
    pub fn get_pixels(
        self,
        face: crate::unity_engine::cubemapface::CubemapFace,
        miplevel: i32,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetPixels", args = 1)]
    pub fn get_pixels_2(
        self,
        face: crate::unity_engine::cubemapface::CubemapFace,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "SetPixels", args = 3)]
    pub fn set_pixels(
        self,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
        face: crate::unity_engine::cubemapface::CubemapFace,
        miplevel: i32,
    ) -> ();

    #[method(name = "SetPixelDataImplArray", args = 6)]
    pub fn set_pixel_data_impl_array(
        self,
        data: ::unity2::IlInstance,
        mip_level: i32,
        face: i32,
        element_size: i32,
        data_array_size: i32,
        source_data_start_index: i32,
    ) -> bool;

    #[method(name = "SetPixelDataImpl", args = 6)]
    pub fn set_pixel_data_impl(
        self,
        data: ::unity2::IntPtr,
        mip_level: i32,
        face: i32,
        element_size: i32,
        data_array_size: i32,
        source_data_start_index: i32,
    ) -> bool;

    #[method(name = "SetPixels", args = 2)]
    pub fn set_pixels_2(
        self,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
        face: crate::unity_engine::cubemapface::CubemapFace,
    ) -> ();

    #[method(name = "GetWritableImageData", args = 1)]
    pub fn get_writable_image_data(self, frame: i32) -> ::unity2::IntPtr;

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

    #[method(name = "get_loadAllMips", args = 0)]
    pub fn get_load_all_mips(self) -> bool;

    #[method(name = "set_loadAllMips", args = 1)]
    pub fn set_load_all_mips(self, value: bool) -> ();

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

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        width: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        width: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        width: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_4(
        self,
        width: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_5(
        self,
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_6(
        self,
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_7(
        self,
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> ();

    #[method(name = "CreateExternalTexture", args = 4)]
    pub fn create_external_texture(
        width: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        mipmap: bool,
        native_tex: ::unity2::IntPtr,
    ) -> crate::unity_engine::cubemap::Cubemap;

    #[method(name = "SetPixel", args = 4)]
    pub fn set_pixel(
        self,
        face: crate::unity_engine::cubemapface::CubemapFace,
        x: i32,
        y: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "GetPixel", args = 3)]
    pub fn get_pixel(
        self,
        face: crate::unity_engine::cubemapface::CubemapFace,
        x: i32,
        y: i32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "Apply", args = 2)]
    pub fn apply(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "Apply", args = 1)]
    pub fn apply_2(self, update_mipmaps: bool) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply_3(self) -> ();

    #[method(name = "ValidateIsNotCrunched", args = 1)]
    pub fn validate_is_not_crunched(
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

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
}

#[cfg(feature = "unity_engine-cubemap")]
impl Cubemap {
    pub fn new(
        width: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new),
            )
        });
        <Self as ICubemapMethods>::ctor(this, width, format, flags);
        this
    }

    pub fn new_2(
        width: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICubemapMethods>::ctor_2(this, width, format, flags);
        this
    }

    pub fn new_3(
        width: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_3),
            )
        });
        <Self as ICubemapMethods>::ctor_3(this, width, format, mip_count);
        this
    }

    pub fn new_4(
        width: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_4),
            )
        });
        <Self as ICubemapMethods>::ctor_4(this, width, format, flags, mip_count);
        this
    }

    pub fn new_5(
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_5),
            )
        });
        <Self as ICubemapMethods>::ctor_5(this, width, texture_format, mip_count, native_tex);
        this
    }

    pub fn new_6(
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_6),
            )
        });
        <Self as ICubemapMethods>::ctor_6(this, width, texture_format, mip_chain, native_tex);
        this
    }

    pub fn new_7(
        width: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Cubemap),
                ::core::stringify!(new_7),
            )
        });
        <Self as ICubemapMethods>::ctor_7(this, width, texture_format, mip_chain);
        this
    }
}
