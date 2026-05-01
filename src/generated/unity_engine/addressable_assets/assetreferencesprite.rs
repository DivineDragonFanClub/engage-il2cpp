
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::addressable_assets::assetreference::AssetReference;
use crate::unity_engine::addressable_assets::assetreference::IAssetReference;
use crate::unity_engine::addressable_assets::assetreferencet_1::AssetReferenceT_1;
use crate::unity_engine::addressable_assets::assetreferencet_1::IAssetReferenceT_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/assetreferencesprite/AssetReferenceSprite.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AssetReferenceSprite"
)]
# [parent (crate :: unity_engine :: addressable_assets :: assetreferencet_1 :: AssetReferenceT_1 < crate :: unity_engine :: sprite :: Sprite >)]
pub struct AssetReferenceSprite {}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencesprite")]
#[::unity2::methods]
impl AssetReferenceSprite {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, guid: ::unity2::Il2CppString) -> ();

    #[method(name = "ValidateAsset", args = 1)]
    pub fn validate_asset(self, path: ::unity2::Il2CppString) -> bool;
}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencesprite")]
impl AssetReferenceSprite {
    pub fn new(guid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetReferenceSprite),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetReferenceSpriteMethods>::ctor(this, guid);
        this
    }
}
