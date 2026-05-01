
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_resourcemanager/TMP_ResourceManager.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_ResourceManager")]
#[parent(crate::system::object::Object)]
pub struct TMP_ResourceManager {
    #[static_field]
    #[rename(name = "s_instance")]
    pub s_instance: crate::tm_pro::tmp_resourcemanager::TMP_ResourceManager,
    #[static_field]
    #[rename(name = "s_TextSettings")]
    pub s_text_settings: crate::tm_pro::tmp_settings::TMP_Settings,
    #[static_field]
    #[rename(name = "s_FontAssetReferences")]
    pub s_font_asset_references: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    >,
    #[static_field]
    #[rename(name = "s_FontAssetReferenceLookup")]
    pub s_font_asset_reference_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            i32,
            crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        >,
}

#[cfg(feature = "tm_pro-tmp_resourcemanager")]
#[::unity2::methods]
impl TMP_ResourceManager {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetTextSettings", args = 0)]
    pub fn get_text_settings() -> crate::tm_pro::tmp_settings::TMP_Settings;

    #[method(name = "AddFontAsset", args = 1)]
    pub fn add_font_asset(font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset) -> ();

    #[method(name = "TryGetFontAsset", args = 2)]
    pub fn try_get_font_asset(
        hashcode: i32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> bool;

    #[method(name = "RebuildFontAssetCache", args = 1)]
    pub fn rebuild_font_asset_cache(instance_id: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_resourcemanager")]
impl TMP_ResourceManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_ResourceManager),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_ResourceManagerMethods>::ctor(this);
        this
    }
}
