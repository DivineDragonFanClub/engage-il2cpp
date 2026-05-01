
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/trackcolorattribute/TrackColorAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TrackColorAttribute")]
pub struct TrackColorAttribute {
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "unity_engine-timeline-trackcolorattribute")]
#[::unity2::methods]
impl TrackColorAttribute {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, r: f32, g: f32, b: f32) -> ();
}

#[cfg(feature = "unity_engine-timeline-trackcolorattribute")]
impl TrackColorAttribute {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackColorAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ITrackColorAttributeMethods>::ctor(this, r, g, b);
        this
    }
}
