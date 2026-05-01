
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationplayableextensions/AnimationPlayableExtensions.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Animations",
    name = "AnimationPlayableExtensions"
)]
#[parent(crate::system::object::Object)]
pub struct AnimationPlayableExtensions {}

#[cfg(feature = "unity_engine-animations-animationplayableextensions")]
#[::unity2::methods]
impl AnimationPlayableExtensions {
    #[method(name = "SetAnimatedPropertiesInternal", args = 2)]
    pub fn set_animated_properties_internal(
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
        animated_properties: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();
}
