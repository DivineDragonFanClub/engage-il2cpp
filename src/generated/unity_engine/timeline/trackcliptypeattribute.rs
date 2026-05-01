
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/trackcliptypeattribute/TrackClipTypeAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TrackClipTypeAttribute")]
pub struct TrackClipTypeAttribute {
    #[rename(name = "inspectedType")]
    pub inspected_type: ::unity2::SystemType,
    #[rename(name = "allowAutoCreate")]
    pub allow_auto_create: bool,
}

#[cfg(feature = "unity_engine-timeline-trackcliptypeattribute")]
#[::unity2::methods]
impl TrackClipTypeAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, clip_class: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, clip_class: ::unity2::SystemType, allow_auto_create: bool) -> ();
}

#[cfg(feature = "unity_engine-timeline-trackcliptypeattribute")]
impl TrackClipTypeAttribute {
    pub fn new(clip_class: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackClipTypeAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ITrackClipTypeAttributeMethods>::ctor(this, clip_class);
        this
    }

    pub fn new_2(clip_class: ::unity2::SystemType, allow_auto_create: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackClipTypeAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITrackClipTypeAttributeMethods>::ctor_2(this, clip_class, allow_auto_create);
        this
    }
}
