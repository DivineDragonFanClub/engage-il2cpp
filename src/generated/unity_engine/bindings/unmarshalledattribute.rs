
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/unmarshalledattribute/UnmarshalledAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "UnmarshalledAttribute")]
pub struct UnmarshalledAttribute {}

#[cfg(feature = "unity_engine-bindings-unmarshalledattribute")]
#[::unity2::methods]
impl UnmarshalledAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-bindings-unmarshalledattribute")]
impl UnmarshalledAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnmarshalledAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IUnmarshalledAttributeMethods>::ctor(this);
        this
    }
}
