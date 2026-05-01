
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/logger_2/Logger_2.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "Logger"
)]
#[parent(crate::system::object::Object)]
pub struct Logger_2 {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-logger_2")]
#[::unity2::methods]
impl Logger_2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-logger_2")]
impl Logger_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Logger_2),
                ::core::stringify!(new),
            )
        });
        <Self as ILogger_2Methods>::ctor(this);
        this
    }
}
