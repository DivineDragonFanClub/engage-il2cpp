
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::markertrack::IMarkerTrack;
use crate::unity_engine::timeline::markertrack::MarkerTrack;
use crate::unity_engine::timeline::trackasset::ITrackAsset;
use crate::unity_engine::timeline::trackasset::TrackAsset;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventmarkertrack/EventMarkerTrack.md")))]
#[::unity2::class(namespace = "App", name = "EventMarkerTrack")]
#[parent(crate::unity_engine::timeline::markertrack::MarkerTrack)]
pub struct EventMarkerTrack {}

#[cfg(feature = "app-eventmarkertrack")]
#[::unity2::methods]
impl EventMarkerTrack {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventmarkertrack")]
impl EventMarkerTrack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMarkerTrack),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMarkerTrackMethods>::ctor(this);
        this
    }
}
