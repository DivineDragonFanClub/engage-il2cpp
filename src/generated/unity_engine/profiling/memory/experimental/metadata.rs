
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/profiling/memory/experimental/metadata/MetaData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Profiling.Memory.Experimental",
    name = "MetaData"
)]
#[parent(crate::system::object::Object)]
pub struct MetaData {
    #[rename(name = "content")]
    pub content: ::unity2::Il2CppString,
    #[rename(name = "platform")]
    pub platform: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-profiling-memory-experimental-metadata")]
#[::unity2::methods]
impl MetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-profiling-memory-experimental-metadata")]
impl MetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IMetaDataMethods>::ctor(this);
        this
    }
}
