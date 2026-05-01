
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::IResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::ResourceProviderBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/assetbundleprovider/AssetBundleProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AssetBundleProvider"
)]
# [parent (crate :: unity_engine :: resource_management :: resource_providers :: resourceproviderbase :: ResourceProviderBase)]
pub struct AssetBundleProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundleprovider")]
#[::unity2::methods]
impl AssetBundleProvider {
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

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundleprovider")]
impl AssetBundleProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleProvider),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleProviderMethods>::ctor(this);
        this
    }
}
