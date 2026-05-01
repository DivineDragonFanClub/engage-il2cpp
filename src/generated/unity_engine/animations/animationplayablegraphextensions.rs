
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationplayablegraphextensions/AnimationPlayableGraphExtensions.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Animations",
    name = "AnimationPlayableGraphExtensions"
)]
#[parent(crate::system::object::Object)]
pub struct AnimationPlayableGraphExtensions {}

#[cfg(feature = "unity_engine-animations-animationplayablegraphextensions")]
#[::unity2::methods]
impl AnimationPlayableGraphExtensions {
    #[method(name = "SyncUpdateAndTimeMode", args = 2)]
    pub fn sync_update_and_time_mode(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        animator: crate::unity_engine::animator::Animator,
    ) -> ();

    #[method(name = "InternalCreateAnimationOutput", args = 3)]
    pub fn internal_create_animation_output(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        name: ::unity2::Il2CppString,
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "InternalSyncUpdateAndTimeMode", args = 2)]
    pub fn internal_sync_update_and_time_mode(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        animator: crate::unity_engine::animator::Animator,
    ) -> ();
}
