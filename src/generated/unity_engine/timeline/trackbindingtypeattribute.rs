
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/trackbindingtypeattribute/TrackBindingTypeAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TrackBindingTypeAttribute")]
pub struct TrackBindingTypeAttribute {
    #[rename(name = "type")]
    pub r#type: ::unity2::SystemType,
    #[rename(name = "flags")]
    pub flags: crate::unity_engine::timeline::trackbindingflags::TrackBindingFlags,
}

#[cfg(feature = "unity_engine-timeline-trackbindingtypeattribute")]
#[::unity2::methods]
impl TrackBindingTypeAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, r#type: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        r#type: ::unity2::SystemType,
        flags: crate::unity_engine::timeline::trackbindingflags::TrackBindingFlags,
    ) -> ();
}

#[cfg(feature = "unity_engine-timeline-trackbindingtypeattribute")]
impl TrackBindingTypeAttribute {
    pub fn new(r#type: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackBindingTypeAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ITrackBindingTypeAttributeMethods>::ctor(this, r#type);
        this
    }

    pub fn new_2(
        r#type: ::unity2::SystemType,
        flags: crate::unity_engine::timeline::trackbindingflags::TrackBindingFlags,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackBindingTypeAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITrackBindingTypeAttributeMethods>::ctor_2(this, r#type, flags);
        this
    }
}
