
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/materialreferencemanager/MaterialReferenceManager.md")))]
#[::unity2::class(namespace = "TMPro", name = "MaterialReferenceManager")]
#[parent(crate::system::object::Object)]
pub struct MaterialReferenceManager {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::tm_pro::materialreferencemanager::MaterialReferenceManager,
    #[rename(name = "m_FontMaterialReferenceLookup")]
    pub m_font_material_reference_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            i32,
            crate::unity_engine::material::Material,
        >,
    #[rename(name = "m_FontAssetReferenceLookup")]
    pub m_font_asset_reference_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            i32,
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
    #[rename(name = "m_SpriteAssetReferenceLookup")]
    pub m_sprite_asset_reference_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            i32,
            crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        >,
    #[rename(name = "m_ColorGradientReferenceLookup")]
    pub m_color_gradient_reference_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            i32,
            crate::tm_pro::tmp_colorgradient::TMP_ColorGradient,
        >,
}

#[cfg(feature = "tm_pro-materialreferencemanager")]
#[::unity2::methods]
impl MaterialReferenceManager {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::tm_pro::materialreferencemanager::MaterialReferenceManager;

    #[method(name = "AddFontAsset", args = 1)]
    pub fn add_font_asset(font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset) -> ();

    #[method(name = "AddFontAssetInternal", args = 1)]
    pub fn add_font_asset_internal(
        self,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> ();

    #[method(name = "AddSpriteAsset", args = 1)]
    pub fn add_sprite_asset(sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset) -> ();

    #[method(name = "AddSpriteAssetInternal", args = 1)]
    pub fn add_sprite_asset_internal(
        self,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    ) -> ();

    #[method(name = "AddSpriteAsset", args = 2)]
    pub fn add_sprite_asset_2(
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    ) -> ();

    #[method(name = "AddSpriteAssetInternal", args = 2)]
    pub fn add_sprite_asset_internal_2(
        self,
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    ) -> ();

    #[method(name = "AddFontMaterial", args = 2)]
    pub fn add_font_material(
        hash_code: i32,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "AddFontMaterialInternal", args = 2)]
    pub fn add_font_material_internal(
        self,
        hash_code: i32,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "AddColorGradientPreset", args = 2)]
    pub fn add_color_gradient_preset(
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_colorgradient::TMP_ColorGradient,
    ) -> ();

    #[method(name = "AddColorGradientPreset_Internal", args = 2)]
    pub fn add_color_gradient_preset_internal(
        self,
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_colorgradient::TMP_ColorGradient,
    ) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, font: crate::tm_pro::tmp_fontasset::TMP_FontAsset) -> bool;

    #[method(name = "Contains", args = 1)]
    pub fn contains_2(self, sprite: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset) -> bool;

    #[method(name = "TryGetFontAsset", args = 2)]
    pub fn try_get_font_asset(
        hash_code: i32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> bool;

    #[method(name = "TryGetFontAssetInternal", args = 2)]
    pub fn try_get_font_asset_internal(
        self,
        hash_code: i32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> bool;

    #[method(name = "TryGetSpriteAsset", args = 2)]
    pub fn try_get_sprite_asset(
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    ) -> bool;

    #[method(name = "TryGetSpriteAssetInternal", args = 2)]
    pub fn try_get_sprite_asset_internal(
        self,
        hash_code: i32,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    ) -> bool;

    #[method(name = "TryGetColorGradientPreset", args = 2)]
    pub fn try_get_color_gradient_preset(
        hash_code: i32,
        gradient_preset: crate::tm_pro::tmp_colorgradient::TMP_ColorGradient,
    ) -> bool;

    #[method(name = "TryGetColorGradientPresetInternal", args = 2)]
    pub fn try_get_color_gradient_preset_internal(
        self,
        hash_code: i32,
        gradient_preset: crate::tm_pro::tmp_colorgradient::TMP_ColorGradient,
    ) -> bool;

    #[method(name = "TryGetMaterial", args = 2)]
    pub fn try_get_material(
        hash_code: i32,
        material: crate::unity_engine::material::Material,
    ) -> bool;

    #[method(name = "TryGetMaterialInternal", args = 2)]
    pub fn try_get_material_internal(
        self,
        hash_code: i32,
        material: crate::unity_engine::material::Material,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-materialreferencemanager")]
impl MaterialReferenceManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialReferenceManager),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialReferenceManagerMethods>::ctor(this);
        this
    }
}
