
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/weightutility/WeightUtility.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "WeightUtility")]
#[parent(crate::system::object::Object)]
pub struct WeightUtility {}

#[cfg(feature = "unity_engine-timeline-weightutility")]
#[::unity2::methods]
impl WeightUtility {
    #[method(name = "NormalizeMixer", args = 1)]
    pub fn normalize_mixer(mixer: crate::unity_engine::playables::playable::Playable) -> f32;
}
