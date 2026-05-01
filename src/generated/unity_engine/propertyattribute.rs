
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/propertyattribute/PropertyAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "PropertyAttribute")]
pub struct PropertyAttribute {}

#[cfg(feature = "unity_engine-propertyattribute")]
#[::unity2::methods]
impl PropertyAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-propertyattribute")]
impl PropertyAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PropertyAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IPropertyAttributeMethods>::ctor(this);
        this
    }
}
