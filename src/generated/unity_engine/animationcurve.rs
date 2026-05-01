
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animationcurve/AnimationCurve.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AnimationCurve")]
#[parent(crate::system::object::Object)]
pub struct AnimationCurve {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-animationcurve")]
#[::unity2::methods]
impl AnimationCurve {
    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(
        keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>,
    ) -> ::unity2::IntPtr;

    #[method(name = "Internal_Equals", args = 1)]
    pub fn internal_equals(self, other: ::unity2::IntPtr) -> bool;

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate(self, time: f32) -> f32;

    #[method(name = "get_keys", args = 0)]
    pub fn get_keys(self) -> ::unity2::Array<crate::unity_engine::keyframe::Keyframe>;

    #[method(name = "set_keys", args = 1)]
    pub fn set_keys(self, value: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>) -> ();

    #[method(name = "AddKey", args = 2)]
    pub fn add_key(self, time: f32, value: f32) -> i32;

    #[method(name = "AddKey", args = 1)]
    pub fn add_key_2(self, key: crate::unity_engine::keyframe::Keyframe) -> i32;

    #[method(name = "AddKey_Internal", args = 1)]
    pub fn add_key_internal(self, key: crate::unity_engine::keyframe::Keyframe) -> i32;

    #[method(name = "MoveKey", args = 2)]
    pub fn move_key(self, index: i32, key: crate::unity_engine::keyframe::Keyframe) -> i32;

    #[method(name = "RemoveKey", args = 1)]
    pub fn remove_key(self, index: i32) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::unity_engine::keyframe::Keyframe;

    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(self, index: i32) -> crate::unity_engine::keyframe::Keyframe;

    #[method(name = "SmoothTangents", args = 2)]
    pub fn smooth_tangents(self, index: i32, weight: f32) -> ();

    #[method(name = "Linear", args = 4)]
    pub fn linear(
        time_start: f32,
        value_start: f32,
        time_end: f32,
        value_end: f32,
    ) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "EaseInOut", args = 4)]
    pub fn ease_in_out(
        time_start: f32,
        value_start: f32,
        time_end: f32,
        value_end: f32,
    ) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "set_preWrapMode", args = 1)]
    pub fn set_pre_wrap_mode(self, value: crate::unity_engine::wrapmode::WrapMode) -> ();

    #[method(name = "set_postWrapMode", args = 1)]
    pub fn set_post_wrap_mode(self, value: crate::unity_engine::wrapmode::WrapMode) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, o: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::animationcurve::AnimationCurve) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "AddKey_Internal_Injected", args = 1)]
    pub fn add_key_internal_injected(self, key: crate::unity_engine::keyframe::Keyframe) -> i32;

    #[method(name = "MoveKey_Injected", args = 2)]
    pub fn move_key_injected(self, index: i32, key: crate::unity_engine::keyframe::Keyframe)
        -> i32;

    #[method(name = "GetKey_Injected", args = 2)]
    pub fn get_key_injected(self, index: i32, ret: crate::unity_engine::keyframe::Keyframe) -> ();
}

#[cfg(feature = "unity_engine-animationcurve")]
impl AnimationCurve {
    pub fn new(keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimationCurve),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimationCurveMethods>::ctor(this, keys);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimationCurve),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAnimationCurveMethods>::ctor_2(this);
        this
    }
}
