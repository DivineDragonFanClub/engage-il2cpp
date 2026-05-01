
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::IResourceProviderBase;
use crate::unity_engine::resource_management::resource_providers::resourceproviderbase::ResourceProviderBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/legacyresourcesprovider/LegacyResourcesProvider_InternalOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "LegacyResourcesProvider.InternalOp"
)]
#[parent(crate::system::object::Object)]
pub struct LegacyResourcesProvider_InternalOp {
    #[rename(name = "m_RequestOperation")]
    pub m_request_operation: crate::unity_engine::asyncoperation::AsyncOperation,
    #[rename(name = "m_ProvideHandle")]
    pub m_provide_handle:
        crate::unity_engine::resource_management::resource_providers::providehandle::ProvideHandle,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-legacyresourcesprovider")]
#[::unity2::methods]
impl LegacyResourcesProvider_InternalOp {
    #[method(name = "Start", args = 1)]
    pub fn start(
        self,
        provide_handle : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "AsyncOperationCompleted", args = 1)]
    pub fn async_operation_completed(
        self,
        op: crate::unity_engine::asyncoperation::AsyncOperation,
    ) -> ();

    #[method(name = "PercentComplete", args = 0)]
    pub fn percent_complete(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-legacyresourcesprovider")]
impl LegacyResourcesProvider_InternalOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LegacyResourcesProvider_InternalOp),
                ::core::stringify!(new),
            )
        });
        <Self as ILegacyResourcesProvider_InternalOpMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/legacyresourcesprovider/LegacyResourcesProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "LegacyResourcesProvider"
)]
# [parent (crate :: unity_engine :: resource_management :: resource_providers :: resourceproviderbase :: ResourceProviderBase)]
pub struct LegacyResourcesProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-legacyresourcesprovider")]
#[::unity2::methods]
impl LegacyResourcesProvider {
    #[method(name = "Provide", args = 1)]
    pub fn provide(
        self,
        pi : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "Release", args = 2)]
    pub fn release(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        asset: crate::system::object::Object,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-legacyresourcesprovider")]
impl LegacyResourcesProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LegacyResourcesProvider),
                ::core::stringify!(new),
            )
        });
        <Self as ILegacyResourcesProviderMethods>::ctor(this);
        this
    }
}
