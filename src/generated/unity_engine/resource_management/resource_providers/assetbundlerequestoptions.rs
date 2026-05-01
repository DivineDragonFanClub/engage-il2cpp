
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/assetbundlerequestoptions/AssetBundleRequestOptions.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AssetBundleRequestOptions"
)]
#[parent(crate::system::object::Object)]
pub struct AssetBundleRequestOptions {
    #[rename(name = "m_Hash")]
    pub m_hash: ::unity2::Il2CppString,
    #[rename(name = "m_Crc")]
    pub m_crc: u32,
    #[rename(name = "m_Timeout")]
    pub m_timeout: i32,
    #[rename(name = "m_ChunkedTransfer")]
    pub m_chunked_transfer: bool,
    #[rename(name = "m_RedirectLimit")]
    pub m_redirect_limit: i32,
    #[rename(name = "m_RetryCount")]
    pub m_retry_count: i32,
    #[rename(name = "m_BundleName")]
    pub m_bundle_name: ::unity2::Il2CppString,
    #[rename(name = "m_BundleSize")]
    pub m_bundle_size: i64,
    #[rename(name = "m_UseCrcForCachedBundles")]
    pub m_use_crc_for_cached_bundles: bool,
    #[rename(name = "m_UseUWRForLocalBundles")]
    pub m_use_uwr_for_local_bundles: bool,
    #[rename(name = "m_ClearOtherCachedVersionsWhenLoaded")]
    pub m_clear_other_cached_versions_when_loaded: bool,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlerequestoptions")]
#[::unity2::methods]
impl AssetBundleRequestOptions {
    #[method(name = "get_Hash", args = 0)]
    pub fn get_hash(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Hash", args = 1)]
    pub fn set_hash(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Crc", args = 0)]
    pub fn get_crc(self) -> u32;

    #[method(name = "set_Crc", args = 1)]
    pub fn set_crc(self, value: u32) -> ();

    #[method(name = "get_Timeout", args = 0)]
    pub fn get_timeout(self) -> i32;

    #[method(name = "set_Timeout", args = 1)]
    pub fn set_timeout(self, value: i32) -> ();

    #[method(name = "get_ChunkedTransfer", args = 0)]
    pub fn get_chunked_transfer(self) -> bool;

    #[method(name = "set_ChunkedTransfer", args = 1)]
    pub fn set_chunked_transfer(self, value: bool) -> ();

    #[method(name = "get_RedirectLimit", args = 0)]
    pub fn get_redirect_limit(self) -> i32;

    #[method(name = "set_RedirectLimit", args = 1)]
    pub fn set_redirect_limit(self, value: i32) -> ();

    #[method(name = "get_RetryCount", args = 0)]
    pub fn get_retry_count(self) -> i32;

    #[method(name = "set_RetryCount", args = 1)]
    pub fn set_retry_count(self, value: i32) -> ();

    #[method(name = "get_BundleName", args = 0)]
    pub fn get_bundle_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BundleName", args = 1)]
    pub fn set_bundle_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BundleSize", args = 0)]
    pub fn get_bundle_size(self) -> i64;

    #[method(name = "set_BundleSize", args = 1)]
    pub fn set_bundle_size(self, value: i64) -> ();

    #[method(name = "get_UseCrcForCachedBundle", args = 0)]
    pub fn get_use_crc_for_cached_bundle(self) -> bool;

    #[method(name = "set_UseCrcForCachedBundle", args = 1)]
    pub fn set_use_crc_for_cached_bundle(self, value: bool) -> ();

    #[method(name = "get_UseUnityWebRequestForLocalBundles", args = 0)]
    pub fn get_use_unity_web_request_for_local_bundles(self) -> bool;

    #[method(name = "set_UseUnityWebRequestForLocalBundles", args = 1)]
    pub fn set_use_unity_web_request_for_local_bundles(self, value: bool) -> ();

    #[method(name = "get_ClearOtherCachedVersionsWhenLoaded", args = 0)]
    pub fn get_clear_other_cached_versions_when_loaded(self) -> bool;

    #[method(name = "set_ClearOtherCachedVersionsWhenLoaded", args = 1)]
    pub fn set_clear_other_cached_versions_when_loaded(self, value: bool) -> ();

    #[method(name = "ComputeSize", args = 2)]
    pub fn compute_size(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager,
    ) -> i64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlerequestoptions")]
impl AssetBundleRequestOptions {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleRequestOptions),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleRequestOptionsMethods>::ctor(this);
        this
    }
}
