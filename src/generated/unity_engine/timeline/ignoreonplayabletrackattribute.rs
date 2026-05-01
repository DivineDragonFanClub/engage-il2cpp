
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/ignoreonplayabletrackattribute/IgnoreOnPlayableTrackAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "IgnoreOnPlayableTrackAttribute"
)]
pub struct IgnoreOnPlayableTrackAttribute {}

#[cfg(feature = "unity_engine-timeline-ignoreonplayabletrackattribute")]
#[::unity2::methods]
impl IgnoreOnPlayableTrackAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-ignoreonplayabletrackattribute")]
impl IgnoreOnPlayableTrackAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IgnoreOnPlayableTrackAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IIgnoreOnPlayableTrackAttributeMethods>::ctor(this);
        this
    }
}
