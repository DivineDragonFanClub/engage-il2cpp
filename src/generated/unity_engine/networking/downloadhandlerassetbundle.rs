
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::networking::downloadhandler::DownloadHandler;
use crate::unity_engine::networking::downloadhandler::IDownloadHandler;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/downloadhandlerassetbundle/DownloadHandlerAssetBundle.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking",
    name = "DownloadHandlerAssetBundle"
)]
#[parent(crate::unity_engine::networking::downloadhandler::DownloadHandler)]
pub struct DownloadHandlerAssetBundle {}

#[cfg(feature = "unity_engine-networking-downloadhandlerassetbundle")]
#[::unity2::methods]
impl DownloadHandlerAssetBundle {
    #[method(name = "Create", args = 3)]
    pub fn create(
        obj : crate :: unity_engine :: networking :: downloadhandlerassetbundle :: DownloadHandlerAssetBundle,
        url: ::unity2::Il2CppString,
        crc: u32,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateCached", args = 5)]
    pub fn create_cached(
        obj : crate :: unity_engine :: networking :: downloadhandlerassetbundle :: DownloadHandlerAssetBundle,
        url: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> ::unity2::IntPtr;

    #[method(name = "InternalCreateAssetBundle", args = 2)]
    pub fn internal_create_asset_bundle(self, url: ::unity2::Il2CppString, crc: u32) -> ();

    #[method(name = "InternalCreateAssetBundleCached", args = 4)]
    pub fn internal_create_asset_bundle_cached(
        self,
        url: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, url: ::unity2::Il2CppString, crc: u32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        url: ::unity2::Il2CppString,
        cached_bundle: crate::unity_engine::cachedassetbundle::CachedAssetBundle,
        crc: u32,
    ) -> ();

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "GetText", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "get_assetBundle", args = 0)]
    pub fn get_asset_bundle(self) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "CreateCached_Injected", args = 5)]
    pub fn create_cached_injected(
        obj : crate :: unity_engine :: networking :: downloadhandlerassetbundle :: DownloadHandlerAssetBundle,
        url: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> ::unity2::IntPtr;
}

#[cfg(feature = "unity_engine-networking-downloadhandlerassetbundle")]
impl DownloadHandlerAssetBundle {
    pub fn new(url: ::unity2::Il2CppString, crc: u32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DownloadHandlerAssetBundle),
                ::core::stringify!(new),
            )
        });
        <Self as IDownloadHandlerAssetBundleMethods>::ctor(this, url, crc);
        this
    }

    pub fn new_2(
        url: ::unity2::Il2CppString,
        cached_bundle: crate::unity_engine::cachedassetbundle::CachedAssetBundle,
        crc: u32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DownloadHandlerAssetBundle),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDownloadHandlerAssetBundleMethods>::ctor_2(this, url, cached_bundle, crc);
        this
    }
}
