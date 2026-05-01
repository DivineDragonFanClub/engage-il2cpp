
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::tm_pro::tmp_textelement::ITMP_TextElement;
use crate::tm_pro::tmp_textelement::TMP_TextElement;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_character/TMP_Character.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Character")]
#[parent(crate::tm_pro::tmp_textelement::TMP_TextElement)]
pub struct TMP_Character {}

#[cfg(feature = "tm_pro-tmp_character")]
#[::unity2::methods]
impl TMP_Character {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, unicode: u32, glyph: crate::unity_engine::text_core::glyph::Glyph) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        unicode: u32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        glyph: crate::unity_engine::text_core::glyph::Glyph,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(self, unicode: u32, glyph_index: u32) -> ();
}

#[cfg(feature = "tm_pro-tmp_character")]
impl TMP_Character {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Character),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_CharacterMethods>::ctor(this);
        this
    }

    pub fn new_2(unicode: u32, glyph: crate::unity_engine::text_core::glyph::Glyph) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Character),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITMP_CharacterMethods>::ctor_2(this, unicode, glyph);
        this
    }

    pub fn new_3(
        unicode: u32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        glyph: crate::unity_engine::text_core::glyph::Glyph,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Character),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITMP_CharacterMethods>::ctor_3(this, unicode, font_asset, glyph);
        this
    }

    pub fn new_4(unicode: u32, glyph_index: u32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Character),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITMP_CharacterMethods>::ctor_4(this, unicode, glyph_index);
        this
    }
}
