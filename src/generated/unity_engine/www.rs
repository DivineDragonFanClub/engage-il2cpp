
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::customyieldinstruction::CustomYieldInstruction;
use crate::unity_engine::customyieldinstruction::ICustomYieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/www/WWW.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "WWW")]
#[parent(crate::unity_engine::customyieldinstruction::CustomYieldInstruction)]
pub struct WWW {
    #[rename(name = "_uwr")]
    pub uwr: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    #[rename(name = "_assetBundle")]
    pub asset_bundle: crate::unity_engine::assetbundle::AssetBundle,
    #[rename(name = "_responseHeaders")]
    pub response_headers: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
}

#[cfg(feature = "unity_engine-www")]
#[::unity2::methods]
impl WWW {
    #[method(name = "LoadFromCacheOrDownload", args = 2)]
    pub fn load_from_cache_or_download(
        url: ::unity2::Il2CppString,
        version: i32,
    ) -> crate::unity_engine::www::WWW;

    #[method(name = "LoadFromCacheOrDownload", args = 3)]
    pub fn load_from_cache_or_download_2(
        url: ::unity2::Il2CppString,
        version: i32,
        crc: u32,
    ) -> crate::unity_engine::www::WWW;

    #[method(name = "LoadFromCacheOrDownload", args = 2)]
    pub fn load_from_cache_or_download_3(
        url: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
    ) -> crate::unity_engine::www::WWW;

    #[method(name = "LoadFromCacheOrDownload", args = 3)]
    pub fn load_from_cache_or_download_4(
        url: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> crate::unity_engine::www::WWW;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        url: ::unity2::Il2CppString,
        form: crate::unity_engine::wwwform::WWWForm,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, url: ::unity2::Il2CppString, post_data: ::unity2::Array<u8>) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        url: ::unity2::Il2CppString,
        post_data: ::unity2::Array<u8>,
        headers: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_4(
        self,
        url: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> ();

    #[method(name = "get_assetBundle", args = 0)]
    pub fn get_asset_bundle(self) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = "get_bytes", args = 0)]
    pub fn get_bytes(self) -> ::unity2::Array<u8>;

    #[method(name = "get_error", args = 0)]
    pub fn get_error(self) -> ::unity2::Il2CppString;

    #[method(name = "get_isDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "get_responseHeaders", args = 0)]
    pub fn get_response_headers(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >;

    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "get_url", args = 0)]
    pub fn get_url(self) -> ::unity2::Il2CppString;

    #[method(name = "get_keepWaiting", args = 0)]
    pub fn get_keep_waiting(self) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "WaitUntilDoneIfPossible", args = 0)]
    pub fn wait_until_done_if_possible(self) -> bool;
}

#[cfg(feature = "unity_engine-www")]
impl WWW {
    pub fn new(url: ::unity2::Il2CppString, form: crate::unity_engine::wwwform::WWWForm) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WWW),
                ::core::stringify!(new),
            )
        });
        <Self as IWWWMethods>::ctor(this, url, form);
        this
    }

    pub fn new_2(url: ::unity2::Il2CppString, post_data: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WWW),
                ::core::stringify!(new_2),
            )
        });
        <Self as IWWWMethods>::ctor_2(this, url, post_data);
        this
    }

    pub fn new_3(
        url: ::unity2::Il2CppString,
        post_data: ::unity2::Array<u8>,
        headers: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WWW),
                ::core::stringify!(new_3),
            )
        });
        <Self as IWWWMethods>::ctor_3(this, url, post_data, headers);
        this
    }

    pub fn new_4(
        url: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
        crc: u32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WWW),
                ::core::stringify!(new_4),
            )
        });
        <Self as IWWWMethods>::ctor_4(this, url, name, hash, crc);
        this
    }
}
