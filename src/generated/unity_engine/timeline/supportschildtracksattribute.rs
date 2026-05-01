
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/supportschildtracksattribute/SupportsChildTracksAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "SupportsChildTracksAttribute"
)]
pub struct SupportsChildTracksAttribute {
    #[rename(name = "childType")]
    pub child_type: ::unity2::SystemType,
    #[rename(name = "levels")]
    pub levels: i32,
}

#[cfg(feature = "unity_engine-timeline-supportschildtracksattribute")]
#[::unity2::methods]
impl SupportsChildTracksAttribute {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, child_type: ::unity2::SystemType, levels: i32) -> ();
}

#[cfg(feature = "unity_engine-timeline-supportschildtracksattribute")]
impl SupportsChildTracksAttribute {
    pub fn new(child_type: ::unity2::SystemType, levels: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SupportsChildTracksAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ISupportsChildTracksAttributeMethods>::ctor(this, child_type, levels);
        this
    }
}
