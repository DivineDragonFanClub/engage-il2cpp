
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/signalargsreaderwriter_jump/SignalArgsReaderWriter_Jump.md")))]
#[::unity2::class(namespace = "Combat", name = "SignalArgsReaderWriter_Jump")]
#[parent(crate::system::object::Object)]
pub struct SignalArgsReaderWriter_Jump {}

#[cfg(feature = "combat-signalargsreaderwriter_jump")]
#[::unity2::methods]
impl SignalArgsReaderWriter_Jump {
    #[method(name = "JumpIsGrounding", args = 1)]
    pub fn jump_is_grounding(ev: crate::unity_engine::animationevent::AnimationEvent) -> bool;

    #[method(name = "JumpIsGrounding", args = 2)]
    pub fn jump_is_grounding_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: bool,
    ) -> ();

    #[method(name = "JumpCurveType", args = 1)]
    pub fn jump_curve_type(
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> crate::app::curve::Curve_Type;

    #[method(name = "JumpCurveType", args = 2)]
    pub fn jump_curve_type_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: crate::app::curve::Curve_Type,
    ) -> ();

    #[method(name = "JumpCurvePower", args = 1)]
    pub fn jump_curve_power(ev: crate::unity_engine::animationevent::AnimationEvent) -> i32;

    #[method(name = "JumpCurvePower", args = 2)]
    pub fn jump_curve_power_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: i32,
    ) -> ();

    #[method(name = "JumpLandingPoint", args = 1)]
    pub fn jump_landing_point(ev: crate::unity_engine::animationevent::AnimationEvent) -> f32;

    #[method(name = "JumpLandingPoint", args = 2)]
    pub fn jump_landing_point_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: f32,
    ) -> ();

    #[method(name = "JumpLandingTimeAfter", args = 1)]
    pub fn jump_landing_time_after(ev: crate::unity_engine::animationevent::AnimationEvent) -> f32;

    #[method(name = "JumpLandingTimeAfter", args = 2)]
    pub fn jump_landing_time_after_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: f32,
    ) -> ();
}
