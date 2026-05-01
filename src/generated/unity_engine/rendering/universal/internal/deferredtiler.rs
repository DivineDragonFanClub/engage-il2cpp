
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/deferredtiler/DeferredTiler_PrePunctualLight.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DeferredTiler_PrePunctualLight {}

impl ::unity2::ClassIdentity for DeferredTiler_PrePunctualLight {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";

    const NAME: &'static str = "DeferredTiler.PrePunctualLight";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DeferredTiler_PrePunctualLight {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/deferredtiler/DeferredTiler_ClipResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DeferredTiler_ClipResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DeferredTiler_ClipResult {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";

    const NAME: &'static str = "DeferredTiler.ClipResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DeferredTiler_ClipResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DeferredTiler_ClipResult {
    pub fn unknown() -> Self {
        Self { value: 0 }
    }

    pub fn r#in() -> Self {
        Self { value: 1 }
    }

    pub fn out() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/deferredtiler/DeferredTiler.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DeferredTiler {}

impl ::unity2::ClassIdentity for DeferredTiler {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";

    const NAME: &'static str = "DeferredTiler";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DeferredTiler {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-universal-internal-deferredtiler")]
#[::unity2::methods(value)]
impl DeferredTiler {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        tile_pixel_width: i32,
        tile_pixel_height: i32,
        avg_light_per_tile: i32,
        tiler_level: i32,
    ) -> ();

    #[method(name = "get_TilerLevel", args = 0)]
    pub fn get_tiler_level(self) -> i32;

    #[method(name = "get_TileXCount", args = 0)]
    pub fn get_tile_x_count(self) -> i32;

    #[method(name = "get_TileYCount", args = 0)]
    pub fn get_tile_y_count(self) -> i32;

    #[method(name = "get_TilePixelWidth", args = 0)]
    pub fn get_tile_pixel_width(self) -> i32;

    #[method(name = "get_TilePixelHeight", args = 0)]
    pub fn get_tile_pixel_height(self) -> i32;

    #[method(name = "get_TileHeaderSize", args = 0)]
    pub fn get_tile_header_size(self) -> i32;

    #[method(name = "get_MaxLightPerTile", args = 0)]
    pub fn get_max_light_per_tile(self) -> i32;

    #[method(name = "get_TileDataCapacity", args = 0)]
    pub fn get_tile_data_capacity(self) -> i32;

    #[method(name = "GetTileOffsetAndCount", args = 4)]
    pub fn get_tile_offset_and_count(self, i: i32, j: i32, offset: i32, count: i32) -> ();

    #[method(name = "GetTileHeaderOffset", args = 2)]
    pub fn get_tile_header_offset(self, i: i32, j: i32) -> i32;

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, tile_data_capacity: i32) -> ();

    #[method(name = "OnCameraCleanup", args = 0)]
    pub fn on_camera_cleanup(self) -> ();

    #[method(name = "PrecomputeTiles", args = 4)]
    pub fn precompute_tiles(
        self,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
        is_orthographic: bool,
        render_width: i32,
        render_height: i32,
    ) -> ();

    #[method(name = "SignedSq", args = 1)]
    pub fn signed_sq(f: f32) -> f32;

    #[method(name = "min2", args = 2)]
    pub fn min2(a: f32, b: f32) -> f32;

    #[method(name = "max2", args = 2)]
    pub fn max2(a: f32, b: f32) -> f32;

    #[method(name = "max3", args = 3)]
    pub fn max3(a: f32, b: f32, c: f32) -> f32;

    #[method(name = "_f32tof16", args = 1)]
    pub fn f32tof16(x: f32) -> u32;

    #[method(name = "Align", args = 2)]
    pub fn align(s: i32, alignment: i32) -> i32;
}
