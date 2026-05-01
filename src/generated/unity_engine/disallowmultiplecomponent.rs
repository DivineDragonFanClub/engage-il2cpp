
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/disallowmultiplecomponent/DisallowMultipleComponent.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "DisallowMultipleComponent")]
pub struct DisallowMultipleComponent {}

#[cfg(feature = "unity_engine-disallowmultiplecomponent")]
#[::unity2::methods]
impl DisallowMultipleComponent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-disallowmultiplecomponent")]
impl DisallowMultipleComponent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisallowMultipleComponent),
                ::core::stringify!(new),
            )
        });
        <Self as IDisallowMultipleComponentMethods>::ctor(this);
        this
    }
}
