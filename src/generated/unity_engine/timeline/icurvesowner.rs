
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/icurvesowner/ICurvesOwner.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "ICurvesOwner")]
pub struct ICurvesOwner {}

#[cfg(feature = "unity_engine-timeline-icurvesowner")]
#[::unity2::methods]
impl ICurvesOwner {
    #[method(name = "get_defaultCurvesName", args = 0)]
    pub fn get_default_curves_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_asset", args = 0)]
    pub fn get_asset(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "get_assetOwner", args = 0)]
    pub fn get_asset_owner(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "get_targetTrack", args = 0)]
    pub fn get_target_track(self) -> crate::unity_engine::timeline::trackasset::TrackAsset;
}
