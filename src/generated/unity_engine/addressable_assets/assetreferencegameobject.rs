
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::addressable_assets::assetreference::AssetReference;
use crate::unity_engine::addressable_assets::assetreference::IAssetReference;
use crate::unity_engine::addressable_assets::assetreferencet_1::AssetReferenceT_1;
use crate::unity_engine::addressable_assets::assetreferencet_1::IAssetReferenceT_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/assetreferencegameobject/AssetReferenceGameObject.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AssetReferenceGameObject"
)]
# [parent (crate :: unity_engine :: addressable_assets :: assetreferencet_1 :: AssetReferenceT_1 < crate :: unity_engine :: gameobject :: GameObject >)]
pub struct AssetReferenceGameObject {}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencegameobject")]
#[::unity2::methods]
impl AssetReferenceGameObject {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, guid: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencegameobject")]
impl AssetReferenceGameObject {
    pub fn new(guid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetReferenceGameObject),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetReferenceGameObjectMethods>::ctor(this, guid);
        this
    }
}
