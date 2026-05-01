
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/exceptions/resourcemanagerexception/ResourceManagerException.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Exceptions",
    name = "ResourceManagerException"
)]
pub struct ResourceManagerException {}

#[cfg(feature = "unity_engine-resource_management-exceptions-resourcemanagerexception")]
#[::unity2::methods]
impl ResourceManagerException {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-resource_management-exceptions-resourcemanagerexception")]
impl ResourceManagerException {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceManagerException),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceManagerExceptionMethods>::ctor(this);
        this
    }

    pub fn new_2(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceManagerException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IResourceManagerExceptionMethods>::ctor_2(this, message);
        this
    }
}
