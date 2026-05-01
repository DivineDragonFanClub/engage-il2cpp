
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/low_level/glyphpairadjustmentrecord/GlyphPairAdjustmentRecord.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GlyphPairAdjustmentRecord {
    pub m_first_adjustment_record:
        crate::unity_engine::text_core::low_level::glyphadjustmentrecord::GlyphAdjustmentRecord,
    pub m_second_adjustment_record:
        crate::unity_engine::text_core::low_level::glyphadjustmentrecord::GlyphAdjustmentRecord,
    pub m_feature_lookup_flags:
        crate::unity_engine::text_core::low_level::fontfeaturelookupflags::FontFeatureLookupFlags,
}

impl ::unity2::ClassIdentity for GlyphPairAdjustmentRecord {
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";

    const NAME: &'static str = "GlyphPairAdjustmentRecord";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphPairAdjustmentRecord {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-text_core-low_level-glyphpairadjustmentrecord")]
#[::unity2::methods(value)]
impl GlyphPairAdjustmentRecord {
    #[method(name = "get_firstAdjustmentRecord", args = 0)]
    pub fn get_first_adjustment_record(
        self,
    ) -> crate::unity_engine::text_core::low_level::glyphadjustmentrecord::GlyphAdjustmentRecord;

    #[method(name = "get_secondAdjustmentRecord", args = 0)]
    pub fn get_second_adjustment_record(
        self,
    ) -> crate::unity_engine::text_core::low_level::glyphadjustmentrecord::GlyphAdjustmentRecord;
}
