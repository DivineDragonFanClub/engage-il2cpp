
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animationeventex/AnimationEventEx.md")))]
#[::unity2::class(namespace = "Combat", name = "AnimationEventEx")]
#[parent(crate::system::object::Object)]
pub struct AnimationEventEx {}

#[cfg(feature = "combat-animationeventex")]
#[::unity2::methods]
impl AnimationEventEx {
    #[method(name = "IsEqualTo", args = 2)]
    pub fn is_equal_to(
        a: crate::unity_engine::animationevent::AnimationEvent,
        b: crate::unity_engine::animationevent::AnimationEvent,
    ) -> bool;

    #[method(name = "IsNotEqualTo", args = 2)]
    pub fn is_not_equal_to(
        a: crate::unity_engine::animationevent::AnimationEvent,
        b: crate::unity_engine::animationevent::AnimationEvent,
    ) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(
        src: crate::unity_engine::animationevent::AnimationEvent,
        dst: crate::unity_engine::animationevent::AnimationEvent,
    ) -> ();
}
