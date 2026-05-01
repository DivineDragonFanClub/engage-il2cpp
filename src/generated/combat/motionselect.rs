
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/motionselect/MotionSelect.md")))]
#[::unity2::class(namespace = "Combat", name = "MotionSelect")]
#[parent(crate::system::object::Object)]
pub struct MotionSelect {}

#[cfg(feature = "combat-motionselect")]
#[::unity2::methods]
impl MotionSelect {
    #[method(name = "SuggestDamage", args = 2)]
    pub fn suggest_damage(
        phase: crate::combat::phase::Phase,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-motionselect")]
impl MotionSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MotionSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IMotionSelectMethods>::ctor(this);
        this
    }
}
