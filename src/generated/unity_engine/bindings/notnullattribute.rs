
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/notnullattribute/NotNullAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "NotNullAttribute")]
pub struct NotNullAttribute {}

#[cfg(feature = "unity_engine-bindings-notnullattribute")]
#[::unity2::methods]
impl NotNullAttribute {
    #[method(name = "set_Exception", args = 1)]
    pub fn set_exception(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, exception: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-bindings-notnullattribute")]
impl NotNullAttribute {
    pub fn new(exception: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NotNullAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INotNullAttributeMethods>::ctor(this, exception);
        this
    }
}
