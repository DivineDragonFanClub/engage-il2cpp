
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/marker/Marker.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "Marker")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct Marker {
    #[rename(name = "m_Time")]
    pub m_time: f64,
}

#[cfg(feature = "unity_engine-timeline-marker")]
#[::unity2::methods]
impl Marker {
    #[method(name = "get_parent", args = 0)]
    pub fn get_parent(self) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "set_parent", args = 1)]
    pub fn set_parent(self, value: crate::unity_engine::timeline::trackasset::TrackAsset) -> ();

    #[method(name = "get_time", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "set_time", args = 1)]
    pub fn set_time(self, value: f64) -> ();

    #[method(name = "UnityEngine.Timeline.IMarker.Initialize", args = 1)]
    pub fn unity_engine_timeline_i_marker_initialize(
        self,
        parent_track: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "OnInitialize", args = 1)]
    pub fn on_initialize(self, a_pent: crate::unity_engine::timeline::trackasset::TrackAsset)
        -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-marker")]
impl Marker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Marker),
                ::core::stringify!(new),
            )
        });
        <Self as IMarkerMethods>::ctor(this);
        this
    }
}
