
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/assetbundle/AssetBundle.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AssetBundle")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct AssetBundle {}

#[cfg(feature = "unity_engine-assetbundle")]
#[::unity2::methods]
impl AssetBundle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "LoadFromFileAsync_Internal", args = 3)]
    pub fn load_from_file_async_internal(
        path: ::unity2::Il2CppString,
        crc: u32,
        offset: u64,
    ) -> crate::unity_engine::assetbundlecreaterequest::AssetBundleCreateRequest;

    #[method(name = "LoadFromFileAsync", args = 1)]
    pub fn load_from_file_async(
        path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::assetbundlecreaterequest::AssetBundleCreateRequest;

    #[method(name = "LoadFromFileAsync", args = 2)]
    pub fn load_from_file_async_2(
        path: ::unity2::Il2CppString,
        crc: u32,
    ) -> crate::unity_engine::assetbundlecreaterequest::AssetBundleCreateRequest;

    #[method(name = "LoadFromFile_Internal", args = 3)]
    pub fn load_from_file_internal(
        path: ::unity2::Il2CppString,
        crc: u32,
        offset: u64,
    ) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "LoadFromFile", args = 2)]
    pub fn load_from_file(
        path: ::unity2::Il2CppString,
        crc: u32,
    ) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "LoadFromMemory_Internal", args = 2)]
    pub fn load_from_memory_internal(
        binary: ::unity2::Array<u8>,
        crc: u32,
    ) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "LoadFromMemory", args = 1)]
    pub fn load_from_memory(
        binary: ::unity2::Array<u8>,
    ) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "LoadAsset", args = 2)]
    pub fn load_asset(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "LoadAsset_Internal", args = 2)]
    pub fn load_asset_internal(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "LoadAssetAsync", args = 2)]
    pub fn load_asset_async(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::assetbundlerequest::AssetBundleRequest;

    #[method(name = "LoadAssetWithSubAssets", args = 2)]
    pub fn load_asset_with_sub_assets(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "LoadAssetWithSubAssetsAsync", args = 2)]
    pub fn load_asset_with_sub_assets_async(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::assetbundlerequest::AssetBundleRequest;

    #[method(name = "LoadAllAssetsAsync", args = 1)]
    pub fn load_all_assets_async(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::assetbundlerequest::AssetBundleRequest;

    #[method(name = "LoadAssetAsync_Internal", args = 2)]
    pub fn load_asset_async_internal(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::assetbundlerequest::AssetBundleRequest;

    #[method(name = "Unload", args = 1)]
    pub fn unload(self, unload_all_loaded_objects: bool) -> ();

    #[method(name = "LoadAssetWithSubAssets_Internal", args = 2)]
    pub fn load_asset_with_sub_assets_internal(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "LoadAssetWithSubAssetsAsync_Internal", args = 2)]
    pub fn load_asset_with_sub_assets_async_internal(
        self,
        name: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::assetbundlerequest::AssetBundleRequest;
}

#[cfg(feature = "unity_engine-assetbundle")]
impl AssetBundle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundle),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleMethods>::ctor(this);
        this
    }
}
