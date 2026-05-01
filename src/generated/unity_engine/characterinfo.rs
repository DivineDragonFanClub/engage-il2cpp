
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/characterinfo/CharacterInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CharacterInfo {
    pub index: i32,
    pub uv: crate::unity_engine::rect::Rect,
    pub vert: crate::unity_engine::rect::Rect,
    pub width: f32,
    pub size: i32,
    pub style: crate::unity_engine::fontstyle::FontStyle,
    pub flipped: bool,
}

impl ::unity2::ClassIdentity for CharacterInfo {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "CharacterInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CharacterInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-characterinfo")]
#[::unity2::methods(value)]
impl CharacterInfo {
    #[method(name = "get_glyphWidth", args = 0)]
    pub fn get_glyph_width(self) -> i32;

    #[method(name = "get_glyphHeight", args = 0)]
    pub fn get_glyph_height(self) -> i32;

    #[method(name = "get_minY", args = 0)]
    pub fn get_min_y(self) -> i32;

    #[method(name = "get_uvBottomLeftUnFlipped", args = 0)]
    pub fn get_uv_bottom_left_un_flipped(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvBottomRightUnFlipped", args = 0)]
    pub fn get_uv_bottom_right_un_flipped(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvTopRightUnFlipped", args = 0)]
    pub fn get_uv_top_right_un_flipped(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvTopLeftUnFlipped", args = 0)]
    pub fn get_uv_top_left_un_flipped(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvBottomLeft", args = 0)]
    pub fn get_uv_bottom_left(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvBottomRight", args = 0)]
    pub fn get_uv_bottom_right(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvTopRight", args = 0)]
    pub fn get_uv_top_right(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_uvTopLeft", args = 0)]
    pub fn get_uv_top_left(self) -> crate::unity_engine::vector2::Vector2;
}
