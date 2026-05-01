
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelinecreateutilities/TimelineCreateUtilities.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimelineCreateUtilities")]
#[parent(crate::system::object::Object)]
pub struct TimelineCreateUtilities {}

#[cfg(feature = "unity_engine-timeline-timelinecreateutilities")]
#[::unity2::methods]
impl TimelineCreateUtilities {
    #[method(name = "GenerateUniqueActorName", args = 2)]
    pub fn generate_unique_actor_name(
        tracks: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::scriptableobject::ScriptableObject,
        >,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SaveAssetIntoObject", args = 2)]
    pub fn save_asset_into_object(
        child_asset: crate::unity_engine::object_2::Object_2,
        master_asset: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "CreateAnimationClipForTrack", args = 3)]
    pub fn create_animation_clip_for_track(
        name: ::unity2::Il2CppString,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        is_legacy: bool,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "ValidateParentTrack", args = 2)]
    pub fn validate_parent_track(
        parent: crate::unity_engine::timeline::trackasset::TrackAsset,
        child_type: ::unity2::SystemType,
    ) -> bool;
}
