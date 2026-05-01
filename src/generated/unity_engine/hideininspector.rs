
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/hideininspector/HideInInspector.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "HideInInspector")]
pub struct HideInInspector {}

#[cfg(feature = "unity_engine-hideininspector")]
#[::unity2::methods]
impl HideInInspector {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-hideininspector")]
impl HideInInspector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HideInInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IHideInInspectorMethods>::ctor(this);
        this
    }
}
