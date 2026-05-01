
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animclipkeyreductionexm/AnimClipKeyReductionExM.md")))]
#[::unity2::class(namespace = "Combat", name = "AnimClipKeyReductionExM")]
#[parent(crate::system::object::Object)]
pub struct AnimClipKeyReductionExM {}

#[cfg(feature = "combat-animclipkeyreductionexm")]
#[::unity2::methods]
impl AnimClipKeyReductionExM {
    #[method(name = "GetValueFromTime", args = 3)]
    pub fn get_value_from_time(
        key1: crate::unity_engine::keyframe::Keyframe,
        key2: crate::unity_engine::keyframe::Keyframe,
        time: f32,
    ) -> f32;

    #[method(name = "IsInterpolationValue", args = 4)]
    pub fn is_interpolation_value(
        key1: crate::unity_engine::keyframe::Keyframe,
        key2: crate::unity_engine::keyframe::Keyframe,
        comp: crate::unity_engine::keyframe::Keyframe,
        eps: f32,
    ) -> bool;

    #[method(name = "GetDeleteKeyIndex", args = 2)]
    pub fn get_delete_key_index(
        keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>,
        eps: f32,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<i32>;

    #[method(name = "KeyReduction", args = 2)]
    pub fn key_reduction(
        in_curve: crate::unity_engine::animationcurve::AnimationCurve,
        eps: f32,
    ) -> ();
}
