
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::addressable_assets::assetreference::AssetReference;
use crate::unity_engine::addressable_assets::assetreference::IAssetReference;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/assetreferencet_1/AssetReferenceT_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AssetReferenceT`1"
)]
pub struct AssetReferenceT_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencet_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> AssetReferenceT_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, guid: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadAsset", args = 0)]
    pub fn load_asset (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > ;

    #[method(name = "LoadAssetAsync", args = 0)]
    pub fn load_asset_async (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > ;

    #[method(name = "ValidateAsset", args = 1)]
    pub fn validate_asset(self, obj: crate::unity_engine::object_2::Object_2) -> bool;

    #[method(name = "ValidateAsset", args = 1)]
    pub fn validate_asset_2(self, main_asset_path: ::unity2::Il2CppString) -> bool;
}

#[cfg(feature = "unity_engine-addressable_assets-assetreferencet_1")]
impl<T0: ::unity2::ClassIdentity> AssetReferenceT_1<T0> {
    pub fn new(guid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetReferenceT_1),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetReferenceT_1Methods<T0>>::ctor(this, guid);
        this
    }
}
