
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/excludefrompresetattribute/ExcludeFromPresetAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ExcludeFromPresetAttribute")]
pub struct ExcludeFromPresetAttribute {}

#[cfg(feature = "unity_engine-excludefrompresetattribute")]
#[::unity2::methods]
impl ExcludeFromPresetAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-excludefrompresetattribute")]
impl ExcludeFromPresetAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExcludeFromPresetAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IExcludeFromPresetAttributeMethods>::ctor(this);
        this
    }
}
