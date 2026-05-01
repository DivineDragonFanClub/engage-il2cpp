
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/glyphrect/GlyphRect.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GlyphRect {
    pub m_x: i32,
    pub m_y: i32,
    pub m_width: i32,
    pub m_height: i32,
}

impl ::unity2::ClassIdentity for GlyphRect {
    const NAMESPACE: &'static str = "UnityEngine.TextCore";

    const NAME: &'static str = "GlyphRect";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphRect {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-text_core-glyphrect")]
#[::unity2::methods(value)]
impl GlyphRect {
    #[method(name = "get_x", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_y", args = 0)]
    pub fn get_y(self) -> i32;

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> i32;

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> i32;

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::text_core::glyphrect::GlyphRect;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x: i32, y: i32, width: i32, height: i32) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::text_core::glyphrect::GlyphRect) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
