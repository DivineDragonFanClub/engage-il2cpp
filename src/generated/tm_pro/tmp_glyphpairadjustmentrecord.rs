
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_glyphpairadjustmentrecord/TMP_GlyphPairAdjustmentRecord.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_GlyphPairAdjustmentRecord")]
#[parent(crate::system::object::Object)]
pub struct TMP_GlyphPairAdjustmentRecord {
    #[rename(name = "m_FirstAdjustmentRecord")]
    pub m_first_adjustment_record:
        crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord,
    #[rename(name = "m_SecondAdjustmentRecord")]
    pub m_second_adjustment_record:
        crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord,
    #[rename(name = "m_FeatureLookupFlags")]
    pub m_feature_lookup_flags: crate::tm_pro::fontfeaturelookupflags_2::FontFeatureLookupFlags_2,
}

#[cfg(feature = "tm_pro-tmp_glyphpairadjustmentrecord")]
#[::unity2::methods]
impl TMP_GlyphPairAdjustmentRecord {
    #[method(name = "get_firstAdjustmentRecord", args = 0)]
    pub fn get_first_adjustment_record(
        self,
    ) -> crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord;

    #[method(name = "set_firstAdjustmentRecord", args = 1)]
    pub fn set_first_adjustment_record(
        self,
        value: crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord,
    ) -> ();

    #[method(name = "get_secondAdjustmentRecord", args = 0)]
    pub fn get_second_adjustment_record(
        self,
    ) -> crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord;

    #[method(name = "set_secondAdjustmentRecord", args = 1)]
    pub fn set_second_adjustment_record(
        self,
        value: crate::tm_pro::tmp_glyphadjustmentrecord::TMP_GlyphAdjustmentRecord,
    ) -> ();

    #[method(name = "get_featureLookupFlags", args = 0)]
    pub fn get_feature_lookup_flags(
        self,
    ) -> crate::tm_pro::fontfeaturelookupflags_2::FontFeatureLookupFlags_2;

    #[method(name = "set_featureLookupFlags", args = 1)]
    pub fn set_feature_lookup_flags(
        self,
        value: crate::tm_pro::fontfeaturelookupflags_2::FontFeatureLookupFlags_2,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        first_adjustment_record : crate :: tm_pro :: tmp_glyphadjustmentrecord :: TMP_GlyphAdjustmentRecord,
        second_adjustment_record : crate :: tm_pro :: tmp_glyphadjustmentrecord :: TMP_GlyphAdjustmentRecord,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        glyph_pair_adjustment_record : crate :: unity_engine :: text_core :: low_level :: glyphpairadjustmentrecord :: GlyphPairAdjustmentRecord,
    ) -> ();
}

#[cfg(feature = "tm_pro-tmp_glyphpairadjustmentrecord")]
impl TMP_GlyphPairAdjustmentRecord {
    pub fn new(
        first_adjustment_record : crate :: tm_pro :: tmp_glyphadjustmentrecord :: TMP_GlyphAdjustmentRecord,
        second_adjustment_record : crate :: tm_pro :: tmp_glyphadjustmentrecord :: TMP_GlyphAdjustmentRecord,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_GlyphPairAdjustmentRecord),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_GlyphPairAdjustmentRecordMethods>::ctor(
            this,
            first_adjustment_record,
            second_adjustment_record,
        );
        this
    }

    pub fn new_2(
        glyph_pair_adjustment_record : crate :: unity_engine :: text_core :: low_level :: glyphpairadjustmentrecord :: GlyphPairAdjustmentRecord,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_GlyphPairAdjustmentRecord),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITMP_GlyphPairAdjustmentRecordMethods>::ctor_2(this, glyph_pair_adjustment_record);
        this
    }
}
