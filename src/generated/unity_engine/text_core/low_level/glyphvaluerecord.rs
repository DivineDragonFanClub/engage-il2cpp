
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/low_level/glyphvaluerecord/GlyphValueRecord.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GlyphValueRecord {
    pub m_x_placement: f32,
    pub m_y_placement: f32,
    pub m_x_advance: f32,
    pub m_y_advance: f32,
}

impl ::unity2::ClassIdentity for GlyphValueRecord {
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";

    const NAME: &'static str = "GlyphValueRecord";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphValueRecord {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-text_core-low_level-glyphvaluerecord")]
#[::unity2::methods(value)]
impl GlyphValueRecord {
    #[method(name = "get_xPlacement", args = 0)]
    pub fn get_x_placement(self) -> f32;

    #[method(name = "get_yPlacement", args = 0)]
    pub fn get_y_placement(self) -> f32;

    #[method(name = "get_xAdvance", args = 0)]
    pub fn get_x_advance(self) -> f32;

    #[method(name = "get_yAdvance", args = 0)]
    pub fn get_y_advance(self) -> f32;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::text_core::low_level::glyphvaluerecord::GlyphValueRecord,
    ) -> bool;
}
