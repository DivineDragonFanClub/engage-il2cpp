
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timeutility/TimeUtility.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimeUtility")]
#[parent(crate::system::object::Object)]
pub struct TimeUtility {
    #[static_field]
    #[rename(name = "kTimeEpsilon")]
    pub k_time_epsilon: f64,
    #[static_field]
    #[rename(name = "kFrameRateEpsilon")]
    pub k_frame_rate_epsilon: f64,
    #[static_field]
    #[rename(name = "k_MaxTimelineDurationInSeconds")]
    pub k_max_timeline_duration_in_seconds: f64,
}

#[cfg(feature = "unity_engine-timeline-timeutility")]
#[::unity2::methods]
impl TimeUtility {
    #[method(name = "GetAnimationClipLength", args = 1)]
    pub fn get_animation_clip_length(
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> f64;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
