
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::texture::ITexture;
use crate::unity_engine::texture::Texture;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/texture3d/Texture3D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Texture3D")]
#[parent(crate::unity_engine::texture::Texture)]
pub struct Texture3D {}

#[cfg(feature = "unity_engine-texture3d")]
#[::unity2::methods]
impl Texture3D {
    #[method(name = "get_isReadable", args = 0)]
    pub fn get_is_readable(self) -> bool;

    #[method(name = "SetPixelImpl", args = 5)]
    pub fn set_pixel_impl(
        self,
        image: i32,
        x: i32,
        y: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "Internal_CreateImpl", args = 8)]
    pub fn internal_create_impl(
        mono: crate::unity_engine::texture3d::Texture3D,
        w: i32,
        h: i32,
        d: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> bool;

    #[method(name = "Internal_Create", args = 8)]
    pub fn internal_create(
        mono: crate::unity_engine::texture3d::Texture3D,
        w: i32,
        h: i32,
        d: i32,
        mip_count: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "ApplyImpl", args = 2)]
    pub fn apply_impl(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "SetPixels", args = 2)]
    pub fn set_pixels(
        self,
        colors: ::unity2::Array<crate::unity_engine::color::Color>,
        miplevel: i32,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_2(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_3(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_4(
        self,
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_5(
        self,
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_6(
        self,
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_7(
        self,
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        native_tex: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "Apply", args = 2)]
    pub fn apply(self, update_mipmaps: bool, make_no_longer_readable: bool) -> ();

    #[method(name = "Apply", args = 1)]
    pub fn apply_2(self, update_mipmaps: bool) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply_3(self) -> ();

    #[method(name = "SetPixel", args = 5)]
    pub fn set_pixel(
        self,
        x: i32,
        y: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
        mip_level: i32,
    ) -> ();

    #[method(name = "ValidateIsNotCrunched", args = 1)]
    pub fn validate_is_not_crunched(
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> ();

    #[method(name = "SetPixelImpl_Injected", args = 5)]
    pub fn set_pixel_impl_injected(
        self,
        image: i32,
        x: i32,
        y: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();
}

#[cfg(feature = "unity_engine-texture3d")]
impl Texture3D {
    pub fn new(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new),
            )
        });
        <Self as ITexture3DMethods>::ctor(this, width, height, depth, format, flags);
        this
    }

    pub fn new_2(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITexture3DMethods>::ctor_2(this, width, height, depth, format, flags);
        this
    }

    pub fn new_3(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        flags : crate :: unity_engine :: experimental :: rendering :: texturecreationflags :: TextureCreationFlags,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITexture3DMethods>::ctor_3(this, width, height, depth, format, flags, mip_count);
        this
    }

    pub fn new_4(
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITexture3DMethods>::ctor_4(this, width, height, depth, texture_format, mip_count);
        this
    }

    pub fn new_5(
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_count: i32,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_5),
            )
        });
        <Self as ITexture3DMethods>::ctor_5(
            this,
            width,
            height,
            depth,
            texture_format,
            mip_count,
            native_tex,
        );
        this
    }

    pub fn new_6(
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_6),
            )
        });
        <Self as ITexture3DMethods>::ctor_6(this, width, height, depth, texture_format, mip_chain);
        this
    }

    pub fn new_7(
        width: i32,
        height: i32,
        depth: i32,
        texture_format: crate::unity_engine::textureformat::TextureFormat,
        mip_chain: bool,
        native_tex: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Texture3D),
                ::core::stringify!(new_7),
            )
        });
        <Self as ITexture3DMethods>::ctor_7(
            this,
            width,
            height,
            depth,
            texture_format,
            mip_chain,
            native_tex,
        );
        this
    }
}
