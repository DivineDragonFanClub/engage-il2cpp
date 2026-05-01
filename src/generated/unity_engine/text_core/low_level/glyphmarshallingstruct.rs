
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/low_level/glyphmarshallingstruct/GlyphMarshallingStruct.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GlyphMarshallingStruct {
    pub index: u32,
    pub metrics: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
    pub glyph_rect: crate::unity_engine::text_core::glyphrect::GlyphRect,
    pub scale: f32,
    pub atlas_index: i32,
}

impl ::unity2::ClassIdentity for GlyphMarshallingStruct {
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";

    const NAME: &'static str = "GlyphMarshallingStruct";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphMarshallingStruct {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
