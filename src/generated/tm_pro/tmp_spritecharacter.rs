
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::tm_pro::tmp_textelement::ITMP_TextElement;
use crate::tm_pro::tmp_textelement::TMP_TextElement;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_spritecharacter/TMP_SpriteCharacter.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_SpriteCharacter")]
#[parent(crate::tm_pro::tmp_textelement::TMP_TextElement)]
pub struct TMP_SpriteCharacter {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_HashCode")]
    pub m_hash_code: i32,
}

#[cfg(feature = "tm_pro-tmp_spritecharacter")]
#[::unity2::methods]
impl TMP_SpriteCharacter {
    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_hashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, unicode: u32, glyph: crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph)
        -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        unicode: u32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        glyph: crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(self, unicode: u32, glyph_index: u32) -> ();
}

#[cfg(feature = "tm_pro-tmp_spritecharacter")]
impl TMP_SpriteCharacter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SpriteCharacter),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_SpriteCharacterMethods>::ctor(this);
        this
    }

    pub fn new_2(unicode: u32, glyph: crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SpriteCharacter),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITMP_SpriteCharacterMethods>::ctor_2(this, unicode, glyph);
        this
    }

    pub fn new_3(
        unicode: u32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        glyph: crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SpriteCharacter),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITMP_SpriteCharacterMethods>::ctor_3(this, unicode, sprite_asset, glyph);
        this
    }

    pub fn new_4(unicode: u32, glyph_index: u32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SpriteCharacter),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITMP_SpriteCharacterMethods>::ctor_4(this, unicode, glyph_index);
        this
    }
}
