
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/threadandserializationsafeattribute/ThreadAndSerializationSafeAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine",
    name = "ThreadAndSerializationSafeAttribute"
)]
pub struct ThreadAndSerializationSafeAttribute {}

#[cfg(feature = "unity_engine-threadandserializationsafeattribute")]
#[::unity2::methods]
impl ThreadAndSerializationSafeAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-threadandserializationsafeattribute")]
impl ThreadAndSerializationSafeAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ThreadAndSerializationSafeAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IThreadAndSerializationSafeAttributeMethods>::ctor(this);
        this
    }
}
