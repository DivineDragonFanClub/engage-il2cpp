
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/ipropertycollector/IPropertyCollector.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "IPropertyCollector")]
pub struct IPropertyCollector {}

#[cfg(feature = "unity_engine-timeline-ipropertycollector")]
#[::unity2::methods]
impl IPropertyCollector {
    #[method(name = "PushActiveGameObject", args = 1)]
    pub fn push_active_game_object(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "PopActiveGameObject", args = 0)]
    pub fn pop_active_game_object(self) -> ();

    #[method(name = "AddFromClip", args = 1)]
    pub fn add_from_clip(self, clip: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(name = "AddObjectProperties", args = 2)]
    pub fn add_object_properties(
        self,
        obj: crate::unity_engine::object_2::Object_2,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();
}
