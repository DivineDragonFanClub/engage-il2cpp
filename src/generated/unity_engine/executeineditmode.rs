
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/executeineditmode/ExecuteInEditMode.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ExecuteInEditMode")]
pub struct ExecuteInEditMode {}

#[cfg(feature = "unity_engine-executeineditmode")]
#[::unity2::methods]
impl ExecuteInEditMode {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-executeineditmode")]
impl ExecuteInEditMode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExecuteInEditMode),
                ::core::stringify!(new),
            )
        });
        <Self as IExecuteInEditModeMethods>::ctor(this);
        this
    }
}
