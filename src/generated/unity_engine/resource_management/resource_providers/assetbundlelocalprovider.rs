
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::IResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::ResourceProviderBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/assetbundlelocalprovider/AssetBundleLocalProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AssetBundleLocalProvider"
)]
# [parent (crate :: unity_engine :: resource_management :: resource_providers :: resourceproviderbase :: ResourceProviderBase)]
pub struct AssetBundleLocalProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlelocalprovider")]
#[::unity2::methods]
impl AssetBundleLocalProvider {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Provide", args = 1)]
    pub fn provide(
        self,
        provider_interface : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "GetDefaultType", args = 1)]
    pub fn get_default_type(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ::unity2::SystemType;

    #[method(name = "Release", args = 2)]
    pub fn release(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        asset: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlelocalprovider")]
impl AssetBundleLocalProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleLocalProvider),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleLocalProviderMethods>::ctor(this);
        this
    }
}
