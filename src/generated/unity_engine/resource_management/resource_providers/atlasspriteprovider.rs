
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::IResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::ResourceProviderBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/atlasspriteprovider/AtlasSpriteProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AtlasSpriteProvider"
)]
# [parent (crate :: unity_engine :: resource_management :: resource_providers :: resourceproviderbase :: ResourceProviderBase)]
pub struct AtlasSpriteProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-atlasspriteprovider")]
#[::unity2::methods]
impl AtlasSpriteProvider {
    #[method(name = "Provide", args = 1)]
    pub fn provide(
        self,
        provider_interface : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-atlasspriteprovider")]
impl AtlasSpriteProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AtlasSpriteProvider),
                ::core::stringify!(new),
            )
        });
        <Self as IAtlasSpriteProviderMethods>::ctor(this);
        this
    }
}
