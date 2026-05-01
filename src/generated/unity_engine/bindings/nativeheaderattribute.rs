
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/nativeheaderattribute/NativeHeaderAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "NativeHeaderAttribute")]
pub struct NativeHeaderAttribute {}

#[cfg(feature = "unity_engine-bindings-nativeheaderattribute")]
#[::unity2::methods]
impl NativeHeaderAttribute {
    #[method(name = "set_Header", args = 1)]
    pub fn set_header(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, header: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-bindings-nativeheaderattribute")]
impl NativeHeaderAttribute {
    pub fn new(header: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeHeaderAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativeHeaderAttributeMethods>::ctor(this, header);
        this
    }
}
