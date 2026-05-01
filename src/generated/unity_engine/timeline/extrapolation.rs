
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/extrapolation/Extrapolation.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "Extrapolation")]
#[parent(crate::system::object::Object)]
pub struct Extrapolation {
    #[static_field]
    #[rename(name = "kMinExtrapolationTime")]
    pub k_min_extrapolation_time: f64,
}

#[cfg(feature = "unity_engine-timeline-extrapolation")]
#[::unity2::methods]
impl Extrapolation {
    #[method(name = "CalculateExtrapolationTimes", args = 1)]
    pub fn calculate_extrapolation_times(
        asset: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "SortClipsByStartTime", args = 1)]
    pub fn sort_clips_by_start_time(
        clips: ::unity2::Array<crate::unity_engine::timeline::timelineclip::TimelineClip>,
    ) -> ::unity2::Array<crate::unity_engine::timeline::timelineclip::TimelineClip>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
