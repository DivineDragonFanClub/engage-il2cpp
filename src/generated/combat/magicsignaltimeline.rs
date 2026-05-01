
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicsignaltimeline/MagicSignalTimeline.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicSignalTimeline")]
#[parent(crate::system::object::Object)]
pub struct MagicSignalTimeline {}

#[cfg(feature = "combat-magicsignaltimeline")]
#[::unity2::methods]
impl MagicSignalTimeline {
    #[method(name = "get_Track", args = 0)]
    pub fn get_track(self) -> crate::combat::magicsignaltrack::MagicSignalTrack;

    #[method(name = "set_Track", args = 1)]
    pub fn set_track(self, value: crate::combat::magicsignaltrack::MagicSignalTrack) -> ();

    #[method(name = "get_PrevTime", args = 0)]
    pub fn get_prev_time(self) -> f32;

    #[method(name = "set_PrevTime", args = 1)]
    pub fn set_prev_time(self, value: f32) -> ();

    #[method(name = "get_Time", args = 0)]
    pub fn get_time(self) -> f32;

    #[method(name = "set_Time", args = 1)]
    pub fn set_time(self, value: f32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, track: crate::combat::magicsignaltrack::MagicSignalTrack) -> ();

    #[method(name = "get_IsEnd", args = 0)]
    pub fn get_is_end(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Advance", args = 1)]
    pub fn advance(self, dt: f32) -> ();
}

#[cfg(feature = "combat-magicsignaltimeline")]
impl MagicSignalTimeline {
    pub fn new(track: crate::combat::magicsignaltrack::MagicSignalTrack) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicSignalTimeline),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicSignalTimelineMethods>::ctor(this, track);
        this
    }
}
