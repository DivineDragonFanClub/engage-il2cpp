
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::tm_pro::tmp_asset::ITMP_Asset;
use crate::tm_pro::tmp_asset::TMP_Asset;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_spriteasset/TMP_SpriteAsset.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_SpriteAsset")]
#[parent(crate::tm_pro::tmp_asset::TMP_Asset)]
pub struct TMP_SpriteAsset {
    #[rename(name = "m_NameLookup")]
    pub m_name_lookup: crate::system::collections::generic::dictionary_2::Dictionary_2<i32, i32>,
    #[rename(name = "m_GlyphIndexLookup")]
    pub m_glyph_index_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<u32, i32>,
    #[rename(name = "m_Version")]
    pub m_version: ::unity2::Il2CppString,
    #[rename(name = "m_FaceInfo")]
    pub m_face_info: crate::unity_engine::text_core::faceinfo::FaceInfo,
    #[rename(name = "spriteSheet")]
    pub sprite_sheet: crate::unity_engine::texture::Texture,
    #[rename(name = "m_SpriteCharacterTable")]
    pub m_sprite_character_table: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
    >,
    #[rename(name = "m_SpriteCharacterLookup")]
    pub m_sprite_character_lookup: crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
    >,
    #[rename(name = "m_SpriteGlyphTable")]
    pub m_sprite_glyph_table: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
    >,
    #[rename(name = "m_SpriteGlyphLookup")]
    pub m_sprite_glyph_lookup: crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
    >,
    #[rename(name = "spriteInfoList")]
    pub sprite_info_list:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_sprite::TMP_Sprite>,
    #[rename(name = "fallbackSpriteAssets")]
    pub fallback_sprite_assets: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    >,
    #[rename(name = "m_IsSpriteAssetLookupTablesDirty")]
    pub m_is_sprite_asset_lookup_tables_dirty: bool,
    #[static_field]
    #[rename(name = "k_searchedSpriteAssets")]
    pub k_searched_sprite_assets: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
}

#[cfg(feature = "tm_pro-tmp_spriteasset")]
#[::unity2::methods]
impl TMP_SpriteAsset {
    #[method(name = "get_version", args = 0)]
    pub fn get_version(self) -> ::unity2::Il2CppString;

    #[method(name = "set_version", args = 1)]
    pub fn set_version(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_faceInfo", args = 0)]
    pub fn get_face_info(self) -> crate::unity_engine::text_core::faceinfo::FaceInfo;

    #[method(name = "set_faceInfo", args = 1)]
    pub fn set_face_info(self, value: crate::unity_engine::text_core::faceinfo::FaceInfo) -> ();

    #[method(name = "get_spriteCharacterTable", args = 0)]
    pub fn get_sprite_character_table(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
    >;

    #[method(name = "set_spriteCharacterTable", args = 1)]
    pub fn set_sprite_character_table(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
        >,
    ) -> ();

    #[method(name = "get_spriteCharacterLookupTable", args = 0)]
    pub fn get_sprite_character_lookup_table(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
    >;

    #[method(name = "set_spriteCharacterLookupTable", args = 1)]
    pub fn set_sprite_character_lookup_table(
        self,
        value: crate::system::collections::generic::dictionary_2::Dictionary_2<
            u32,
            crate::tm_pro::tmp_spritecharacter::TMP_SpriteCharacter,
        >,
    ) -> ();

    #[method(name = "get_spriteGlyphTable", args = 0)]
    pub fn get_sprite_glyph_table(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
    >;

    #[method(name = "set_spriteGlyphTable", args = 1)]
    pub fn set_sprite_glyph_table(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_spriteglyph::TMP_SpriteGlyph,
        >,
    ) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "GetDefaultSpriteMaterial", args = 0)]
    pub fn get_default_sprite_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "UpdateLookupTables", args = 0)]
    pub fn update_lookup_tables(self) -> ();

    #[method(name = "GetSpriteIndexFromHashcode", args = 1)]
    pub fn get_sprite_index_from_hashcode(self, hash_code: i32) -> i32;

    #[method(name = "GetSpriteIndexFromUnicode", args = 1)]
    pub fn get_sprite_index_from_unicode(self, unicode: u32) -> i32;

    #[method(name = "GetSpriteIndexFromName", args = 1)]
    pub fn get_sprite_index_from_name(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "SearchForSpriteByUnicode", args = 4)]
    pub fn search_for_sprite_by_unicode(
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        unicode: u32,
        include_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SearchForSpriteByUnicodeInternal", args = 4)]
    pub fn search_for_sprite_by_unicode_internal(
        sprite_assets: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        >,
        unicode: u32,
        include_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SearchForSpriteByUnicodeInternal", args = 4)]
    pub fn search_for_sprite_by_unicode_internal_2(
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        unicode: u32,
        include_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SearchForSpriteByHashCode", args = 4)]
    pub fn search_for_sprite_by_hash_code(
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        hash_code: i32,
        include_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SearchForSpriteByHashCodeInternal", args = 4)]
    pub fn search_for_sprite_by_hash_code_internal(
        sprite_assets: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        >,
        hash_code: i32,
        search_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SearchForSpriteByHashCodeInternal", args = 4)]
    pub fn search_for_sprite_by_hash_code_internal_2(
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        hash_code: i32,
        search_fallbacks: bool,
        sprite_index: i32,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "SortGlyphTable", args = 0)]
    pub fn sort_glyph_table(self) -> ();

    #[method(name = "SortCharacterTable", args = 0)]
    pub fn sort_character_table(self) -> ();

    #[method(name = "SortGlyphAndCharacterTables", args = 0)]
    pub fn sort_glyph_and_character_tables(self) -> ();

    #[method(name = "UpgradeSpriteAsset", args = 0)]
    pub fn upgrade_sprite_asset(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_spriteasset")]
impl TMP_SpriteAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SpriteAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_SpriteAssetMethods>::ctor(this);
        this
    }
}
