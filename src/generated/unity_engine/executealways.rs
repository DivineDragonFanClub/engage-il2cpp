
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/executealways/ExecuteAlways.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ExecuteAlways")]
pub struct ExecuteAlways {}

#[cfg(feature = "unity_engine-executealways")]
#[::unity2::methods]
impl ExecuteAlways {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-executealways")]
impl ExecuteAlways {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExecuteAlways),
                ::core::stringify!(new),
            )
        });
        <Self as IExecuteAlwaysMethods>::ctor(this);
        this
    }
}
