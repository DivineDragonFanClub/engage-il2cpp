
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_fontutilities/TMP_FontUtilities.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_FontUtilities")]
#[parent(crate::system::object::Object)]
pub struct TMP_FontUtilities {
    #[static_field]
    #[rename(name = "k_searchedFontAssets")]
    pub k_searched_font_assets: crate::system::collections::generic::list_1::List_1<i32>,
}

#[cfg(feature = "tm_pro-tmp_fontutilities")]
#[::unity2::methods]
impl TMP_FontUtilities {
    #[method(name = "SearchForCharacter", args = 3)]
    pub fn search_for_character(
        font: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "SearchForCharacter", args = 3)]
    pub fn search_for_character_2(
        fonts: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "SearchForCharacterInternal", args = 3)]
    pub fn search_for_character_internal(
        font: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "SearchForCharacterInternal", args = 3)]
    pub fn search_for_character_internal_2(
        fonts: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;
}
