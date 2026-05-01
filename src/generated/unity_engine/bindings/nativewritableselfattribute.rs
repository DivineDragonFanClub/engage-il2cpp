
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/nativewritableselfattribute/NativeWritableSelfAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Bindings",
    name = "NativeWritableSelfAttribute"
)]
pub struct NativeWritableSelfAttribute {}

#[cfg(feature = "unity_engine-bindings-nativewritableselfattribute")]
#[::unity2::methods]
impl NativeWritableSelfAttribute {
    #[method(name = "set_WritableSelf", args = 1)]
    pub fn set_writable_self(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-bindings-nativewritableselfattribute")]
impl NativeWritableSelfAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeWritableSelfAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativeWritableSelfAttributeMethods>::ctor(this);
        this
    }
}
