
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/providerloadrequestoptions/ProviderLoadRequestOptions.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "ProviderLoadRequestOptions"
)]
#[parent(crate::system::object::Object)]
pub struct ProviderLoadRequestOptions {
    #[rename(name = "m_IgnoreFailures")]
    pub m_ignore_failures: bool,
    #[rename(name = "m_WebRequestTimeout")]
    pub m_web_request_timeout: i32,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-providerloadrequestoptions")]
#[::unity2::methods]
impl ProviderLoadRequestOptions {
    #[method(name = "Copy", args = 0)]
    pub fn copy (self ,) -> crate :: unity_engine :: resource_management :: resource_providers :: providerloadrequestoptions :: ProviderLoadRequestOptions ;

    #[method(name = "get_IgnoreFailures", args = 0)]
    pub fn get_ignore_failures(self) -> bool;

    #[method(name = "set_IgnoreFailures", args = 1)]
    pub fn set_ignore_failures(self, value: bool) -> ();

    #[method(name = "get_WebRequestTimeout", args = 0)]
    pub fn get_web_request_timeout(self) -> i32;

    #[method(name = "set_WebRequestTimeout", args = 1)]
    pub fn set_web_request_timeout(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-providerloadrequestoptions")]
impl ProviderLoadRequestOptions {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProviderLoadRequestOptions),
                ::core::stringify!(new),
            )
        });
        <Self as IProviderLoadRequestOptionsMethods>::ctor(this);
        this
    }
}
