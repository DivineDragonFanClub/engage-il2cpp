
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_settings/TMP_Settings.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Settings")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct TMP_Settings {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::tm_pro::tmp_settings::TMP_Settings,
    #[rename(name = "m_enableWordWrapping")]
    pub m_enable_word_wrapping: bool,
    #[rename(name = "m_enableKerning")]
    pub m_enable_kerning: bool,
    #[rename(name = "m_enableExtraPadding")]
    pub m_enable_extra_padding: bool,
    #[rename(name = "m_enableTintAllSprites")]
    pub m_enable_tint_all_sprites: bool,
    #[rename(name = "m_enableParseEscapeCharacters")]
    pub m_enable_parse_escape_characters: bool,
    #[rename(name = "m_EnableRaycastTarget")]
    pub m_enable_raycast_target: bool,
    #[rename(name = "m_GetFontFeaturesAtRuntime")]
    pub m_get_font_features_at_runtime: bool,
    #[rename(name = "m_missingGlyphCharacter")]
    pub m_missing_glyph_character: i32,
    #[rename(name = "m_warningsDisabled")]
    pub m_warnings_disabled: bool,
    #[rename(name = "m_defaultFontAsset")]
    pub m_default_font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    #[rename(name = "m_defaultFontAssetPath")]
    pub m_default_font_asset_path: ::unity2::Il2CppString,
    #[rename(name = "m_defaultFontSize")]
    pub m_default_font_size: f32,
    #[rename(name = "m_defaultAutoSizeMinRatio")]
    pub m_default_auto_size_min_ratio: f32,
    #[rename(name = "m_defaultAutoSizeMaxRatio")]
    pub m_default_auto_size_max_ratio: f32,
    #[rename(name = "m_defaultTextMeshProTextContainerSize")]
    pub m_default_text_mesh_pro_text_container_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_defaultTextMeshProUITextContainerSize")]
    pub m_default_text_mesh_pro_ui_text_container_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_autoSizeTextContainer")]
    pub m_auto_size_text_container: bool,
    #[rename(name = "m_IsTextObjectScaleStatic")]
    pub m_is_text_object_scale_static: bool,
    #[rename(name = "m_fallbackFontAssets")]
    pub m_fallback_font_assets: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >,
    #[rename(name = "m_matchMaterialPreset")]
    pub m_match_material_preset: bool,
    #[rename(name = "m_defaultSpriteAsset")]
    pub m_default_sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    #[rename(name = "m_defaultSpriteAssetPath")]
    pub m_default_sprite_asset_path: ::unity2::Il2CppString,
    #[rename(name = "m_enableEmojiSupport")]
    pub m_enable_emoji_support: bool,
    #[rename(name = "m_MissingCharacterSpriteUnicode")]
    pub m_missing_character_sprite_unicode: u32,
    #[rename(name = "m_defaultColorGradientPresetsPath")]
    pub m_default_color_gradient_presets_path: ::unity2::Il2CppString,
    #[rename(name = "m_defaultStyleSheet")]
    pub m_default_style_sheet: crate::tm_pro::tmp_stylesheet::TMP_StyleSheet,
    #[rename(name = "m_StyleSheetsResourcePath")]
    pub m_style_sheets_resource_path: ::unity2::Il2CppString,
    #[rename(name = "m_leadingCharacters")]
    pub m_leading_characters: crate::unity_engine::textasset::TextAsset,
    #[rename(name = "m_followingCharacters")]
    pub m_following_characters: crate::unity_engine::textasset::TextAsset,
    #[rename(name = "m_linebreakingRules")]
    pub m_linebreaking_rules: crate::tm_pro::tmp_settings::TMP_Settings_LineBreakingTable,
    #[rename(name = "m_UseModernHangulLineBreakingRules")]
    pub m_use_modern_hangul_line_breaking_rules: bool,
}

#[cfg(feature = "tm_pro-tmp_settings")]
#[::unity2::methods]
impl TMP_Settings {
    #[method(name = "get_version", args = 0)]
    pub fn get_version() -> ::unity2::Il2CppString;

    #[method(name = "get_enableWordWrapping", args = 0)]
    pub fn get_enable_word_wrapping() -> bool;

    #[method(name = "get_enableKerning", args = 0)]
    pub fn get_enable_kerning() -> bool;

    #[method(name = "get_enableExtraPadding", args = 0)]
    pub fn get_enable_extra_padding() -> bool;

    #[method(name = "get_enableTintAllSprites", args = 0)]
    pub fn get_enable_tint_all_sprites() -> bool;

    #[method(name = "get_enableParseEscapeCharacters", args = 0)]
    pub fn get_enable_parse_escape_characters() -> bool;

    #[method(name = "get_enableRaycastTarget", args = 0)]
    pub fn get_enable_raycast_target() -> bool;

    #[method(name = "get_getFontFeaturesAtRuntime", args = 0)]
    pub fn get_get_font_features_at_runtime() -> bool;

    #[method(name = "get_missingGlyphCharacter", args = 0)]
    pub fn get_missing_glyph_character() -> i32;

    #[method(name = "set_missingGlyphCharacter", args = 1)]
    pub fn set_missing_glyph_character(value: i32) -> ();

    #[method(name = "get_warningsDisabled", args = 0)]
    pub fn get_warnings_disabled() -> bool;

    #[method(name = "get_defaultFontAsset", args = 0)]
    pub fn get_default_font_asset() -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "get_defaultFontAssetPath", args = 0)]
    pub fn get_default_font_asset_path() -> ::unity2::Il2CppString;

    #[method(name = "get_defaultFontSize", args = 0)]
    pub fn get_default_font_size() -> f32;

    #[method(name = "get_defaultTextAutoSizingMinRatio", args = 0)]
    pub fn get_default_text_auto_sizing_min_ratio() -> f32;

    #[method(name = "get_defaultTextAutoSizingMaxRatio", args = 0)]
    pub fn get_default_text_auto_sizing_max_ratio() -> f32;

    #[method(name = "get_defaultTextMeshProTextContainerSize", args = 0)]
    pub fn get_default_text_mesh_pro_text_container_size() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_defaultTextMeshProUITextContainerSize", args = 0)]
    pub fn get_default_text_mesh_pro_ui_text_container_size(
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_autoSizeTextContainer", args = 0)]
    pub fn get_auto_size_text_container() -> bool;

    #[method(name = "get_isTextObjectScaleStatic", args = 0)]
    pub fn get_is_text_object_scale_static() -> bool;

    #[method(name = "set_isTextObjectScaleStatic", args = 1)]
    pub fn set_is_text_object_scale_static(value: bool) -> ();

    #[method(name = "get_fallbackFontAssets", args = 0)]
    pub fn get_fallback_font_assets() -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >;

    #[method(name = "get_matchMaterialPreset", args = 0)]
    pub fn get_match_material_preset() -> bool;

    #[method(name = "get_defaultSpriteAsset", args = 0)]
    pub fn get_default_sprite_asset() -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "get_defaultSpriteAssetPath", args = 0)]
    pub fn get_default_sprite_asset_path() -> ::unity2::Il2CppString;

    #[method(name = "get_enableEmojiSupport", args = 0)]
    pub fn get_enable_emoji_support() -> bool;

    #[method(name = "set_enableEmojiSupport", args = 1)]
    pub fn set_enable_emoji_support(value: bool) -> ();

    #[method(name = "get_missingCharacterSpriteUnicode", args = 0)]
    pub fn get_missing_character_sprite_unicode() -> u32;

    #[method(name = "set_missingCharacterSpriteUnicode", args = 1)]
    pub fn set_missing_character_sprite_unicode(value: u32) -> ();

    #[method(name = "get_defaultColorGradientPresetsPath", args = 0)]
    pub fn get_default_color_gradient_presets_path() -> ::unity2::Il2CppString;

    #[method(name = "get_defaultStyleSheet", args = 0)]
    pub fn get_default_style_sheet() -> crate::tm_pro::tmp_stylesheet::TMP_StyleSheet;

    #[method(name = "get_styleSheetsResourcePath", args = 0)]
    pub fn get_style_sheets_resource_path() -> ::unity2::Il2CppString;

    #[method(name = "get_leadingCharacters", args = 0)]
    pub fn get_leading_characters() -> crate::unity_engine::textasset::TextAsset;

    #[method(name = "get_followingCharacters", args = 0)]
    pub fn get_following_characters() -> crate::unity_engine::textasset::TextAsset;

    #[method(name = "get_linebreakingRules", args = 0)]
    pub fn get_linebreaking_rules() -> crate::tm_pro::tmp_settings::TMP_Settings_LineBreakingTable;

    #[method(name = "get_useModernHangulLineBreakingRules", args = 0)]
    pub fn get_use_modern_hangul_line_breaking_rules() -> bool;

    #[method(name = "set_useModernHangulLineBreakingRules", args = 1)]
    pub fn set_use_modern_hangul_line_breaking_rules(value: bool) -> ();

    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::tm_pro::tmp_settings::TMP_Settings;

    #[method(name = "LoadDefaultSettings", args = 0)]
    pub fn load_default_settings() -> crate::tm_pro::tmp_settings::TMP_Settings;

    #[method(name = "GetSettings", args = 0)]
    pub fn get_settings() -> crate::tm_pro::tmp_settings::TMP_Settings;

    #[method(name = "GetFontAsset", args = 0)]
    pub fn get_font_asset() -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "GetSpriteAsset", args = 0)]
    pub fn get_sprite_asset() -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "GetStyleSheet", args = 0)]
    pub fn get_style_sheet() -> crate::tm_pro::tmp_stylesheet::TMP_StyleSheet;

    #[method(name = "LoadLinebreakingRules", args = 0)]
    pub fn load_linebreaking_rules() -> ();

    #[method(name = "GetCharacters", args = 1)]
    pub fn get_characters(
        file: crate::unity_engine::textasset::TextAsset,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<i32, u16>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_settings")]
impl TMP_Settings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Settings),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_SettingsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_settings/TMP_Settings_LineBreakingTable.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Settings.LineBreakingTable")]
#[parent(crate::system::object::Object)]
pub struct TMP_Settings_LineBreakingTable {
    #[rename(name = "leadingCharacters")]
    pub leading_characters:
        crate::system::collections::generic::dictionary_2::Dictionary_2<i32, u16>,
    #[rename(name = "followingCharacters")]
    pub following_characters:
        crate::system::collections::generic::dictionary_2::Dictionary_2<i32, u16>,
}

#[cfg(feature = "tm_pro-tmp_settings")]
#[::unity2::methods]
impl TMP_Settings_LineBreakingTable {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_settings")]
impl TMP_Settings_LineBreakingTable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Settings_LineBreakingTable),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_Settings_LineBreakingTableMethods>::ctor(this);
        this
    }
}
