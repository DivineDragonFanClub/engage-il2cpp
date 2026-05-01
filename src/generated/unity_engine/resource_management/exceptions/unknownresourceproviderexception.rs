
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::exceptions::resourcemanagerexception::IResourceManagerException;
use crate::unity_engine::resource_management::exceptions::resourcemanagerexception::ResourceManagerException;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/exceptions/unknownresourceproviderexception/UnknownResourceProviderException.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Exceptions",
    name = "UnknownResourceProviderException"
)]
# [parent (crate :: unity_engine :: resource_management :: exceptions :: resourcemanagerexception :: ResourceManagerException)]
pub struct UnknownResourceProviderException {}

#[cfg(feature = "unity_engine-resource_management-exceptions-unknownresourceproviderexception")]
#[::unity2::methods]
impl UnknownResourceProviderException {
    #[method(name = "get_Location", args = 0)]
    pub fn get_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;

    #[method(name = "set_Location", args = 1)]
    pub fn set_location(
        self,
        value : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Message", args = 0)]
    pub fn get_message(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-resource_management-exceptions-unknownresourceproviderexception")]
impl UnknownResourceProviderException {
    pub fn new(
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnknownResourceProviderException),
                ::core::stringify!(new),
            )
        });
        <Self as IUnknownResourceProviderExceptionMethods>::ctor(this, location);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnknownResourceProviderException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IUnknownResourceProviderExceptionMethods>::ctor_2(this);
        this
    }

    pub fn new_3(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnknownResourceProviderException),
                ::core::stringify!(new_3),
            )
        });
        <Self as IUnknownResourceProviderExceptionMethods>::ctor_3(this, message);
        this
    }
}
