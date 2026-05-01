
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/internal/excludefromdocsattribute/ExcludeFromDocsAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Internal", name = "ExcludeFromDocsAttribute")]
pub struct ExcludeFromDocsAttribute {}

#[cfg(feature = "unity_engine-internal-excludefromdocsattribute")]
#[::unity2::methods]
impl ExcludeFromDocsAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-internal-excludefromdocsattribute")]
impl ExcludeFromDocsAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExcludeFromDocsAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IExcludeFromDocsAttributeMethods>::ctor(this);
        this
    }
}
