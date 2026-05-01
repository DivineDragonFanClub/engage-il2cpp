
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::tm_pro::tmp_asset::ITMP_Asset;
use crate::tm_pro::tmp_asset::TMP_Asset;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_fontasset/TMP_FontAsset.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_FontAsset")]
#[parent(crate::tm_pro::tmp_asset::TMP_Asset)]
pub struct TMP_FontAsset {
    #[rename(name = "m_Version")]
    pub m_version: ::unity2::Il2CppString,
    #[rename(name = "m_SourceFontFileGUID")]
    pub m_source_font_file_guid: ::unity2::Il2CppString,
    #[rename(name = "m_SourceFontFile")]
    pub m_source_font_file: crate::unity_engine::font::Font,
    #[rename(name = "m_AtlasPopulationMode")]
    pub m_atlas_population_mode: crate::tm_pro::atlaspopulationmode::AtlasPopulationMode,
    #[rename(name = "m_FaceInfo")]
    pub m_face_info: crate::unity_engine::text_core::faceinfo::FaceInfo,
    #[rename(name = "m_GlyphTable")]
    pub m_glyph_table: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyph::Glyph,
    >,
    #[rename(name = "m_GlyphLookupDictionary")]
    pub m_glyph_lookup_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::unity_engine::text_core::glyph::Glyph,
    >,
    #[rename(name = "m_CharacterTable")]
    pub m_character_table: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_character::TMP_Character,
    >,
    #[rename(name = "m_CharacterLookupDictionary")]
    pub m_character_lookup_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            u32,
            crate::tm_pro::tmp_character::TMP_Character,
        >,
    #[rename(name = "m_AtlasTexture")]
    pub m_atlas_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_AtlasTextures")]
    pub m_atlas_textures: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
    #[rename(name = "m_AtlasTextureIndex")]
    pub m_atlas_texture_index: i32,
    #[rename(name = "m_IsMultiAtlasTexturesEnabled")]
    pub m_is_multi_atlas_textures_enabled: bool,
    #[rename(name = "m_ClearDynamicDataOnBuild")]
    pub m_clear_dynamic_data_on_build: bool,
    #[rename(name = "m_UsedGlyphRects")]
    pub m_used_glyph_rects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyphrect::GlyphRect,
    >,
    #[rename(name = "m_FreeGlyphRects")]
    pub m_free_glyph_rects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyphrect::GlyphRect,
    >,
    #[rename(name = "m_fontInfo")]
    pub m_font_info: crate::tm_pro::faceinfo_legacy::FaceInfo_Legacy,
    #[rename(name = "atlas")]
    pub atlas: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_AtlasWidth")]
    pub m_atlas_width: i32,
    #[rename(name = "m_AtlasHeight")]
    pub m_atlas_height: i32,
    #[rename(name = "m_AtlasPadding")]
    pub m_atlas_padding: i32,
    #[rename(name = "m_AtlasRenderMode")]
    pub m_atlas_render_mode:
        crate::unity_engine::text_core::low_level::glyphrendermode::GlyphRenderMode,
    #[rename(name = "m_glyphInfoList")]
    pub m_glyph_info_list:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_glyph::TMP_Glyph>,
    #[rename(name = "m_KerningTable")]
    pub m_kerning_table: crate::tm_pro::kerningtable::KerningTable,
    #[rename(name = "m_FontFeatureTable")]
    pub m_font_feature_table: crate::tm_pro::tmp_fontfeaturetable::TMP_FontFeatureTable,
    #[rename(name = "fallbackFontAssets")]
    pub fallback_font_assets: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >,
    #[rename(name = "m_FallbackFontAssetTable")]
    pub m_fallback_font_asset_table: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >,
    #[rename(name = "m_CreationSettings")]
    pub m_creation_settings: crate::tm_pro::fontassetcreationsettings::FontAssetCreationSettings,
    #[rename(name = "m_FontWeightTable")]
    pub m_font_weight_table: ::unity2::Array<crate::tm_pro::tmp_fontweightpair::TMP_FontWeightPair>,
    #[rename(name = "fontWeights")]
    pub font_weights: ::unity2::Array<crate::tm_pro::tmp_fontweightpair::TMP_FontWeightPair>,
    #[rename(name = "normalStyle")]
    pub normal_style: f32,
    #[rename(name = "normalSpacingOffset")]
    pub normal_spacing_offset: f32,
    #[rename(name = "boldStyle")]
    pub bold_style: f32,
    #[rename(name = "boldSpacing")]
    pub bold_spacing: f32,
    #[rename(name = "italicStyle")]
    pub italic_style: u8,
    #[rename(name = "tabSize")]
    pub tab_size: u8,
    #[rename(name = "IsFontAssetLookupTablesDirty")]
    pub is_font_asset_lookup_tables_dirty: bool,
    #[static_field]
    #[rename(name = "s_DefaultMaterialSuffix")]
    pub s_default_material_suffix: ::unity2::Il2CppString,
    #[rename(name = "FallbackSearchQueryLookup")]
    pub fallback_search_query_lookup:
        crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[static_field]
    #[rename(name = "k_SearchedFontAssetLookup")]
    pub k_searched_font_asset_lookup:
        crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[static_field]
    #[rename(name = "k_FontAssets_FontFeaturesUpdateQueue")]
    pub k_font_assets_font_features_update_queue:
        crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
    #[static_field]
    #[rename(name = "k_FontAssets_FontFeaturesUpdateQueueLookup")]
    pub k_font_assets_font_features_update_queue_lookup:
        crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[static_field]
    #[rename(name = "k_FontAssets_AtlasTexturesUpdateQueue")]
    pub k_font_assets_atlas_textures_update_queue:
        crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
    #[static_field]
    #[rename(name = "k_FontAssets_AtlasTexturesUpdateQueueLookup")]
    pub k_font_assets_atlas_textures_update_queue_lookup:
        crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_GlyphsToRender")]
    pub m_glyphs_to_render: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyph::Glyph,
    >,
    #[rename(name = "m_GlyphsRendered")]
    pub m_glyphs_rendered: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyph::Glyph,
    >,
    #[rename(name = "m_GlyphIndexList")]
    pub m_glyph_index_list: crate::system::collections::generic::list_1::List_1<u32>,
    #[rename(name = "m_GlyphIndexListNewlyAdded")]
    pub m_glyph_index_list_newly_added: crate::system::collections::generic::list_1::List_1<u32>,
    #[rename(name = "m_GlyphsToAdd")]
    pub m_glyphs_to_add: crate::system::collections::generic::list_1::List_1<u32>,
    #[rename(name = "m_GlyphsToAddLookup")]
    pub m_glyphs_to_add_lookup: crate::system::collections::generic::hashset_1::HashSet_1<u32>,
    #[rename(name = "m_CharactersToAdd")]
    pub m_characters_to_add: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_character::TMP_Character,
    >,
    #[rename(name = "m_CharactersToAddLookup")]
    pub m_characters_to_add_lookup: crate::system::collections::generic::hashset_1::HashSet_1<u32>,
    #[rename(name = "s_MissingCharacterList")]
    pub s_missing_character_list: crate::system::collections::generic::list_1::List_1<u32>,
    #[rename(name = "m_MissingUnicodesFromFontFile")]
    pub m_missing_unicodes_from_font_file:
        crate::system::collections::generic::hashset_1::HashSet_1<u32>,
    #[static_field]
    #[rename(name = "k_GlyphIndexArray")]
    pub k_glyph_index_array: ::unity2::Array<u32>,
}

#[cfg(feature = "tm_pro-tmp_fontasset")]
#[::unity2::methods]
impl TMP_FontAsset {
    #[method(name = "get_version", args = 0)]
    pub fn get_version(self) -> ::unity2::Il2CppString;

    #[method(name = "set_version", args = 1)]
    pub fn set_version(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_sourceFontFile", args = 0)]
    pub fn get_source_font_file(self) -> crate::unity_engine::font::Font;

    #[method(name = "set_sourceFontFile", args = 1)]
    pub fn set_source_font_file(self, value: crate::unity_engine::font::Font) -> ();

    #[method(name = "get_atlasPopulationMode", args = 0)]
    pub fn get_atlas_population_mode(
        self,
    ) -> crate::tm_pro::atlaspopulationmode::AtlasPopulationMode;

    #[method(name = "set_atlasPopulationMode", args = 1)]
    pub fn set_atlas_population_mode(
        self,
        value: crate::tm_pro::atlaspopulationmode::AtlasPopulationMode,
    ) -> ();

    #[method(name = "get_faceInfo", args = 0)]
    pub fn get_face_info(self) -> crate::unity_engine::text_core::faceinfo::FaceInfo;

    #[method(name = "set_faceInfo", args = 1)]
    pub fn set_face_info(self, value: crate::unity_engine::text_core::faceinfo::FaceInfo) -> ();

    #[method(name = "get_glyphTable", args = 0)]
    pub fn get_glyph_table(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyph::Glyph,
    >;

    #[method(name = "set_glyphTable", args = 1)]
    pub fn set_glyph_table(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::text_core::glyph::Glyph,
        >,
    ) -> ();

    #[method(name = "get_glyphLookupTable", args = 0)]
    pub fn get_glyph_lookup_table(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::unity_engine::text_core::glyph::Glyph,
    >;

    #[method(name = "get_characterTable", args = 0)]
    pub fn get_character_table(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_character::TMP_Character,
    >;

    #[method(name = "set_characterTable", args = 1)]
    pub fn set_character_table(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_character::TMP_Character,
        >,
    ) -> ();

    #[method(name = "get_characterLookupTable", args = 0)]
    pub fn get_character_lookup_table(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        u32,
        crate::tm_pro::tmp_character::TMP_Character,
    >;

    #[method(name = "get_atlasTexture", args = 0)]
    pub fn get_atlas_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_atlasTextures", args = 0)]
    pub fn get_atlas_textures(self) -> ::unity2::Array<crate::unity_engine::texture2d::Texture2D>;

    #[method(name = "set_atlasTextures", args = 1)]
    pub fn set_atlas_textures(
        self,
        value: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
    ) -> ();

    #[method(name = "get_atlasTextureCount", args = 0)]
    pub fn get_atlas_texture_count(self) -> i32;

    #[method(name = "get_isMultiAtlasTexturesEnabled", args = 0)]
    pub fn get_is_multi_atlas_textures_enabled(self) -> bool;

    #[method(name = "set_isMultiAtlasTexturesEnabled", args = 1)]
    pub fn set_is_multi_atlas_textures_enabled(self, value: bool) -> ();

    #[method(name = "get_clearDynamicDataOnBuild", args = 0)]
    pub fn get_clear_dynamic_data_on_build(self) -> bool;

    #[method(name = "set_clearDynamicDataOnBuild", args = 1)]
    pub fn set_clear_dynamic_data_on_build(self, value: bool) -> ();

    #[method(name = "get_usedGlyphRects", args = 0)]
    pub fn get_used_glyph_rects(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyphrect::GlyphRect,
    >;

    #[method(name = "set_usedGlyphRects", args = 1)]
    pub fn set_used_glyph_rects(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::text_core::glyphrect::GlyphRect,
        >,
    ) -> ();

    #[method(name = "get_freeGlyphRects", args = 0)]
    pub fn get_free_glyph_rects(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::text_core::glyphrect::GlyphRect,
    >;

    #[method(name = "set_freeGlyphRects", args = 1)]
    pub fn set_free_glyph_rects(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::text_core::glyphrect::GlyphRect,
        >,
    ) -> ();

    #[method(name = "get_fontInfo", args = 0)]
    pub fn get_font_info(self) -> crate::tm_pro::faceinfo_legacy::FaceInfo_Legacy;

    #[method(name = "get_atlasWidth", args = 0)]
    pub fn get_atlas_width(self) -> i32;

    #[method(name = "set_atlasWidth", args = 1)]
    pub fn set_atlas_width(self, value: i32) -> ();

    #[method(name = "get_atlasHeight", args = 0)]
    pub fn get_atlas_height(self) -> i32;

    #[method(name = "set_atlasHeight", args = 1)]
    pub fn set_atlas_height(self, value: i32) -> ();

    #[method(name = "get_atlasPadding", args = 0)]
    pub fn get_atlas_padding(self) -> i32;

    #[method(name = "set_atlasPadding", args = 1)]
    pub fn set_atlas_padding(self, value: i32) -> ();

    #[method(name = "get_atlasRenderMode", args = 0)]
    pub fn get_atlas_render_mode(
        self,
    ) -> crate::unity_engine::text_core::low_level::glyphrendermode::GlyphRenderMode;

    #[method(name = "set_atlasRenderMode", args = 1)]
    pub fn set_atlas_render_mode(
        self,
        value: crate::unity_engine::text_core::low_level::glyphrendermode::GlyphRenderMode,
    ) -> ();

    #[method(name = "get_fontFeatureTable", args = 0)]
    pub fn get_font_feature_table(
        self,
    ) -> crate::tm_pro::tmp_fontfeaturetable::TMP_FontFeatureTable;

    #[method(name = "set_fontFeatureTable", args = 1)]
    pub fn set_font_feature_table(
        self,
        value: crate::tm_pro::tmp_fontfeaturetable::TMP_FontFeatureTable,
    ) -> ();

    #[method(name = "get_fallbackFontAssetTable", args = 0)]
    pub fn get_fallback_font_asset_table(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >;

    #[method(name = "set_fallbackFontAssetTable", args = 1)]
    pub fn set_fallback_font_asset_table(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
    ) -> ();

    #[method(name = "get_creationSettings", args = 0)]
    pub fn get_creation_settings(
        self,
    ) -> crate::tm_pro::fontassetcreationsettings::FontAssetCreationSettings;

    #[method(name = "set_creationSettings", args = 1)]
    pub fn set_creation_settings(
        self,
        value: crate::tm_pro::fontassetcreationsettings::FontAssetCreationSettings,
    ) -> ();

    #[method(name = "get_fontWeightTable", args = 0)]
    pub fn get_font_weight_table(
        self,
    ) -> ::unity2::Array<crate::tm_pro::tmp_fontweightpair::TMP_FontWeightPair>;

    #[method(name = "set_fontWeightTable", args = 1)]
    pub fn set_font_weight_table(
        self,
        value: ::unity2::Array<crate::tm_pro::tmp_fontweightpair::TMP_FontWeightPair>,
    ) -> ();

    #[method(name = "CreateFontAsset", args = 1)]
    pub fn create_font_asset(
        font: crate::unity_engine::font::Font,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "CreateFontAsset", args = 8)]
    pub fn create_font_asset_2(
        font: crate::unity_engine::font::Font,
        sampling_point_size: i32,
        atlas_padding: i32,
        render_mode: crate::unity_engine::text_core::low_level::glyphrendermode::GlyphRenderMode,
        atlas_width: i32,
        atlas_height: i32,
        atlas_population_mode: crate::tm_pro::atlaspopulationmode::AtlasPopulationMode,
        enable_multi_atlas_support: bool,
    ) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "ReadFontAssetDefinition", args = 0)]
    pub fn read_font_asset_definition(self) -> ();

    #[method(name = "InitializeDictionaryLookupTables", args = 0)]
    pub fn initialize_dictionary_lookup_tables(self) -> ();

    #[method(name = "InitializeGlyphLookupDictionary", args = 0)]
    pub fn initialize_glyph_lookup_dictionary(self) -> ();

    #[method(name = "InitializeCharacterLookupDictionary", args = 0)]
    pub fn initialize_character_lookup_dictionary(self) -> ();

    #[method(
        name = "InitializeGlyphPaidAdjustmentRecordsLookupDictionary",
        args = 0
    )]
    pub fn initialize_glyph_paid_adjustment_records_lookup_dictionary(self) -> ();

    #[method(name = "AddSynthesizedCharactersAndFaceMetrics", args = 0)]
    pub fn add_synthesized_characters_and_face_metrics(self) -> ();

    #[method(name = "AddSynthesizedCharacter", args = 3)]
    pub fn add_synthesized_character(
        self,
        unicode: u32,
        is_font_face_loaded: bool,
        add_immediately: bool,
    ) -> ();

    #[method(name = "AddCharacterToLookupCache", args = 2)]
    pub fn add_character_to_lookup_cache(
        self,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> ();

    #[method(name = "SortCharacterTable", args = 0)]
    pub fn sort_character_table(self) -> ();

    #[method(name = "SortGlyphTable", args = 0)]
    pub fn sort_glyph_table(self) -> ();

    #[method(name = "SortFontFeatureTable", args = 0)]
    pub fn sort_font_feature_table(self) -> ();

    #[method(name = "SortAllTables", args = 0)]
    pub fn sort_all_tables(self) -> ();

    #[method(name = "HasCharacter", args = 1)]
    pub fn has_character(self, character: i32) -> bool;

    #[method(name = "HasCharacter", args = 3)]
    pub fn has_character_2(
        self,
        character: u16,
        search_fallbacks: bool,
        try_add_character: bool,
    ) -> bool;

    #[method(name = "HasCharacter_Internal", args = 3)]
    pub fn has_character_internal(
        self,
        character: u32,
        search_fallbacks: bool,
        try_add_character: bool,
    ) -> bool;

    #[method(name = "HasCharacters", args = 2)]
    pub fn has_characters(
        self,
        text: ::unity2::Il2CppString,
        missing_characters: crate::system::collections::generic::list_1::List_1<u16>,
    ) -> bool;

    #[method(name = "HasCharacters", args = 4)]
    pub fn has_characters_2(
        self,
        text: ::unity2::Il2CppString,
        missing_characters: ::unity2::Array<u32>,
        search_fallbacks: bool,
        try_add_character: bool,
    ) -> bool;

    #[method(name = "HasCharacters", args = 1)]
    pub fn has_characters_3(self, text: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetCharacters", args = 1)]
    pub fn get_characters(
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetCharactersArray", args = 1)]
    pub fn get_characters_array(
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> ::unity2::Array<i32>;

    #[method(name = "GetGlyphIndex", args = 1)]
    pub fn get_glyph_index(self, unicode: u32) -> u32;

    #[method(name = "RegisterFontAssetForFontFeatureUpdate", args = 1)]
    pub fn register_font_asset_for_font_feature_update(
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> ();

    #[method(name = "UpdateFontFeaturesForFontAssetsInQueue", args = 0)]
    pub fn update_font_features_for_font_assets_in_queue() -> ();

    #[method(name = "RegisterFontAssetForAtlasTextureUpdate", args = 1)]
    pub fn register_font_asset_for_atlas_texture_update(
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> ();

    #[method(name = "UpdateAtlasTexturesForFontAssetsInQueue", args = 0)]
    pub fn update_atlas_textures_for_font_assets_in_queue() -> ();

    #[method(name = "TryAddCharacters", args = 2)]
    pub fn try_add_characters(
        self,
        unicodes: ::unity2::Array<u32>,
        include_font_features: bool,
    ) -> bool;

    #[method(name = "TryAddCharacters", args = 3)]
    pub fn try_add_characters_2(
        self,
        unicodes: ::unity2::Array<u32>,
        missing_unicodes: ::unity2::Array<u32>,
        include_font_features: bool,
    ) -> bool;

    #[method(name = "TryAddCharacters", args = 2)]
    pub fn try_add_characters_3(
        self,
        characters: ::unity2::Il2CppString,
        include_font_features: bool,
    ) -> bool;

    #[method(name = "TryAddCharacters", args = 3)]
    pub fn try_add_characters_4(
        self,
        characters: ::unity2::Il2CppString,
        missing_characters: ::unity2::Il2CppString,
        include_font_features: bool,
    ) -> bool;

    #[method(name = "TryAddCharacterInternal", args = 2)]
    pub fn try_add_character_internal(
        self,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> bool;

    #[method(name = "TryGetCharacter_and_QueueRenderToTexture", args = 2)]
    pub fn try_get_character_and_queue_render_to_texture(
        self,
        unicode: u32,
        character: crate::tm_pro::tmp_character::TMP_Character,
    ) -> bool;

    #[method(name = "TryAddGlyphsToAtlasTextures", args = 0)]
    pub fn try_add_glyphs_to_atlas_textures(self) -> ();

    #[method(name = "TryAddGlyphsToNewAtlasTexture", args = 0)]
    pub fn try_add_glyphs_to_new_atlas_texture(self) -> bool;

    #[method(name = "SetupNewAtlasTexture", args = 0)]
    pub fn setup_new_atlas_texture(self) -> ();

    #[method(name = "UpdateAtlasTexture", args = 0)]
    pub fn update_atlas_texture(self) -> ();

    #[method(name = "UpdateGlyphAdjustmentRecords", args = 0)]
    pub fn update_glyph_adjustment_records(self) -> ();

    #[method(name = "UpdateGlyphAdjustmentRecords", args = 1)]
    pub fn update_glyph_adjustment_records_2(self, glyph_indexes: ::unity2::Array<u32>) -> ();

    #[method(name = "UpdateGlyphAdjustmentRecords", args = 1)]
    pub fn update_glyph_adjustment_records_3(
        self,
        glyph_indexes: crate::system::collections::generic::list_1::List_1<u32>,
    ) -> ();

    #[method(name = "UpdateGlyphAdjustmentRecords", args = 2)]
    pub fn update_glyph_adjustment_records_4(
        self,
        new_glyph_indexes: crate::system::collections::generic::list_1::List_1<u32>,
        all_glyph_indexes: crate::system::collections::generic::list_1::List_1<u32>,
    ) -> ();

    #[method(name = "ClearFontAssetData", args = 1)]
    pub fn clear_font_asset_data(self, set_atlas_size_to_zero: bool) -> ();

    #[method(name = "ClearFontAssetDataInternal", args = 0)]
    pub fn clear_font_asset_data_internal(self) -> ();

    #[method(name = "UpdateFontAssetData", args = 0)]
    pub fn update_font_asset_data(self) -> ();

    #[method(name = "ClearFontAssetTables", args = 0)]
    pub fn clear_font_asset_tables(self) -> ();

    #[method(name = "ClearAtlasTextures", args = 1)]
    pub fn clear_atlas_textures(self, set_atlas_size_to_zero: bool) -> ();

    #[method(name = "UpgradeFontAsset", args = 0)]
    pub fn upgrade_font_asset(self) -> ();

    #[method(name = "UpgradeGlyphAdjustmentTableToFontFeatureTable", args = 0)]
    pub fn upgrade_glyph_adjustment_table_to_font_feature_table(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-tmp_fontasset")]
impl TMP_FontAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_FontAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_FontAssetMethods>::ctor(this);
        this
    }
}
