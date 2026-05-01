
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::IResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::ResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::textdataprovider::ITextDataProvider;
use crate::unity_engine::resource_management::resource_providers::textdataprovider::TextDataProvider;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/jsonassetprovider/JsonAssetProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "JsonAssetProvider"
)]
# [parent (crate :: unity_engine :: resource_management :: resource_providers :: textdataprovider :: TextDataProvider)]
pub struct JsonAssetProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-jsonassetprovider")]
#[::unity2::methods]
impl JsonAssetProvider {
    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        r#type: ::unity2::SystemType,
        text: ::unity2::Il2CppString,
    ) -> crate::system::object::Object;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-jsonassetprovider")]
impl JsonAssetProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JsonAssetProvider),
                ::core::stringify!(new),
            )
        });
        <Self as IJsonAssetProviderMethods>::ctor(this);
        this
    }
}
