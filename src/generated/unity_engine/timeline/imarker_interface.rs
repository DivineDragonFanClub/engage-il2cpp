
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/imarker_interface/IMarker_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "IMarker")]
pub struct IMarker_Interface {}

#[cfg(feature = "unity_engine-timeline-imarker_interface")]
#[::unity2::methods]
impl IMarker_Interface {
    #[method(name = "get_time", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "set_time", args = 1)]
    pub fn set_time(self, value: f64) -> ();

    #[method(name = "get_parent", args = 0)]
    pub fn get_parent(self) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, parent: crate::unity_engine::timeline::trackasset::TrackAsset) -> ();
}
