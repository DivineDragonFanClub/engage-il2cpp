
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/glyphmetrics/GlyphMetrics.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GlyphMetrics {
    pub m_width: f32,
    pub m_height: f32,
    pub m_horizontal_bearing_x: f32,
    pub m_horizontal_bearing_y: f32,
    pub m_horizontal_advance: f32,
}

impl ::unity2::ClassIdentity for GlyphMetrics {
    const NAMESPACE: &'static str = "UnityEngine.TextCore";

    const NAME: &'static str = "GlyphMetrics";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphMetrics {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-text_core-glyphmetrics")]
#[::unity2::methods(value)]
impl GlyphMetrics {
    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "get_horizontalBearingX", args = 0)]
    pub fn get_horizontal_bearing_x(self) -> f32;

    #[method(name = "get_horizontalBearingY", args = 0)]
    pub fn get_horizontal_bearing_y(self) -> f32;

    #[method(name = "get_horizontalAdvance", args = 0)]
    pub fn get_horizontal_advance(self) -> f32;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(self, width: f32, height: f32, bearing_x: f32, bearing_y: f32, advance: f32) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
    ) -> bool;
}
