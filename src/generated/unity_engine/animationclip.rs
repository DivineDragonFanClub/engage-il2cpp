
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::motion::IMotion;
use crate::unity_engine::motion::Motion;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animationclip/AnimationClip.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AnimationClip")]
#[parent(crate::unity_engine::motion::Motion)]
pub struct AnimationClip {}

#[cfg(feature = "unity_engine-animationclip")]
#[::unity2::methods]
impl AnimationClip {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Internal_CreateAnimationClip", args = 1)]
    pub fn internal_create_animation_clip(
        self_: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "SampleAnimation", args = 2)]
    pub fn sample_animation(self, go: crate::unity_engine::gameobject::GameObject, time: f32)
        -> ();

    #[method(name = "SampleAnimation", args = 4)]
    pub fn sample_animation_2(
        go: crate::unity_engine::gameobject::GameObject,
        clip: crate::unity_engine::animationclip::AnimationClip,
        in_time: f32,
        wrap_mode: crate::unity_engine::wrapmode::WrapMode,
    ) -> ();

    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> f32;

    #[method(name = "get_startTime", args = 0)]
    pub fn get_start_time(self) -> f32;

    #[method(name = "get_stopTime", args = 0)]
    pub fn get_stop_time(self) -> f32;

    #[method(name = "get_frameRate", args = 0)]
    pub fn get_frame_rate(self) -> f32;

    #[method(name = "set_frameRate", args = 1)]
    pub fn set_frame_rate(self, value: f32) -> ();

    #[method(name = "SetCurve", args = 4)]
    pub fn set_curve(
        self,
        relative_path: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
        property_name: ::unity2::Il2CppString,
        curve: crate::unity_engine::animationcurve::AnimationCurve,
    ) -> ();

    #[method(name = "EnsureQuaternionContinuity", args = 0)]
    pub fn ensure_quaternion_continuity(self) -> ();

    #[method(name = "ClearCurves", args = 0)]
    pub fn clear_curves(self) -> ();

    #[method(name = "get_wrapMode", args = 0)]
    pub fn get_wrap_mode(self) -> crate::unity_engine::wrapmode::WrapMode;

    #[method(name = "set_wrapMode", args = 1)]
    pub fn set_wrap_mode(self, value: crate::unity_engine::wrapmode::WrapMode) -> ();

    #[method(name = "get_localBounds", args = 0)]
    pub fn get_local_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "set_localBounds", args = 1)]
    pub fn set_local_bounds(self, value: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "get_legacy", args = 0)]
    pub fn get_legacy(self) -> bool;

    #[method(name = "set_legacy", args = 1)]
    pub fn set_legacy(self, value: bool) -> ();

    #[method(name = "get_humanMotion", args = 0)]
    pub fn get_human_motion(self) -> bool;

    #[method(name = "get_empty", args = 0)]
    pub fn get_empty(self) -> bool;

    #[method(name = "get_hasGenericRootTransform", args = 0)]
    pub fn get_has_generic_root_transform(self) -> bool;

    #[method(name = "get_hasMotionFloatCurves", args = 0)]
    pub fn get_has_motion_float_curves(self) -> bool;

    #[method(name = "get_hasMotionCurves", args = 0)]
    pub fn get_has_motion_curves(self) -> bool;

    #[method(name = "get_hasRootCurves", args = 0)]
    pub fn get_has_root_curves(self) -> bool;

    #[method(name = "get_hasRootMotion", args = 0)]
    pub fn get_has_root_motion(self) -> bool;

    #[method(name = "AddEvent", args = 1)]
    pub fn add_event(self, evt: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "AddEventInternal", args = 1)]
    pub fn add_event_internal(self, evt: crate::system::object::Object) -> ();

    #[method(name = "get_events", args = 0)]
    pub fn get_events(self)
        -> ::unity2::Array<crate::unity_engine::animationevent::AnimationEvent>;

    #[method(name = "set_events", args = 1)]
    pub fn set_events(
        self,
        value: ::unity2::Array<crate::unity_engine::animationevent::AnimationEvent>,
    ) -> ();

    #[method(name = "SetEventsInternal", args = 1)]
    pub fn set_events_internal(self, value: ::unity2::IlInstance) -> ();

    #[method(name = "GetEventsInternal", args = 0)]
    pub fn get_events_internal(self) -> ::unity2::IlInstance;

    #[method(name = "get_localBounds_Injected", args = 1)]
    pub fn get_local_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "set_localBounds_Injected", args = 1)]
    pub fn set_local_bounds_injected(self, value: crate::unity_engine::bounds::Bounds) -> ();
}

#[cfg(feature = "unity_engine-animationclip")]
impl AnimationClip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimationClip),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimationClipMethods>::ctor(this);
        this
    }
}
