
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/kerningpair/KerningPair.md")))]
#[::unity2::class(namespace = "TMPro", name = "KerningPair")]
#[parent(crate::system::object::Object)]
pub struct KerningPair {
    #[rename(name = "m_FirstGlyph")]
    pub m_first_glyph: u32,
    #[rename(name = "m_FirstGlyphAdjustments")]
    pub m_first_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
    #[rename(name = "m_SecondGlyph")]
    pub m_second_glyph: u32,
    #[rename(name = "m_SecondGlyphAdjustments")]
    pub m_second_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
    #[rename(name = "xOffset")]
    pub x_offset: f32,
    #[static_field]
    #[rename(name = "empty")]
    pub empty: crate::tm_pro::kerningpair::KerningPair,
    #[rename(name = "m_IgnoreSpacingAdjustments")]
    pub m_ignore_spacing_adjustments: bool,
}

#[cfg(feature = "tm_pro-kerningpair")]
#[::unity2::methods]
impl KerningPair {
    #[method(name = "get_firstGlyph", args = 0)]
    pub fn get_first_glyph(self) -> u32;

    #[method(name = "set_firstGlyph", args = 1)]
    pub fn set_first_glyph(self, value: u32) -> ();

    #[method(name = "get_firstGlyphAdjustments", args = 0)]
    pub fn get_first_glyph_adjustments(
        self,
    ) -> crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy;

    #[method(name = "get_secondGlyph", args = 0)]
    pub fn get_second_glyph(self) -> u32;

    #[method(name = "set_secondGlyph", args = 1)]
    pub fn set_second_glyph(self, value: u32) -> ();

    #[method(name = "get_secondGlyphAdjustments", args = 0)]
    pub fn get_second_glyph_adjustments(
        self,
    ) -> crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy;

    #[method(name = "get_ignoreSpacingAdjustments", args = 0)]
    pub fn get_ignore_spacing_adjustments(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(self, left: u32, right: u32, offset: f32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_3(
        self,
        first_glyph: u32,
        first_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
        second_glyph: u32,
        second_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
    ) -> ();

    #[method(name = "ConvertLegacyKerningData", args = 0)]
    pub fn convert_legacy_kerning_data(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-kerningpair")]
impl KerningPair {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KerningPair),
                ::core::stringify!(new),
            )
        });
        <Self as IKerningPairMethods>::ctor(this);
        this
    }

    pub fn new_2(left: u32, right: u32, offset: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KerningPair),
                ::core::stringify!(new_2),
            )
        });
        <Self as IKerningPairMethods>::ctor_2(this, left, right, offset);
        this
    }

    pub fn new_3(
        first_glyph: u32,
        first_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
        second_glyph: u32,
        second_glyph_adjustments: crate::tm_pro::glyphvaluerecord_legacy::GlyphValueRecord_Legacy,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KerningPair),
                ::core::stringify!(new_3),
            )
        });
        <Self as IKerningPairMethods>::ctor_3(
            this,
            first_glyph,
            first_glyph_adjustments,
            second_glyph,
            second_glyph_adjustments,
        );
        this
    }
}
