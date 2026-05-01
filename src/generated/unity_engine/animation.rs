
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animation/Animation_Enumerator.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Animation.Enumerator")]
#[parent(crate::system::object::Object)]
pub struct Animation_Enumerator {
    #[rename(name = "m_Outer")]
    pub m_outer: crate::unity_engine::animation::Animation,
    #[rename(name = "m_CurrentIndex")]
    pub m_current_index: i32,
}

#[cfg(feature = "unity_engine-animation")]
#[::unity2::methods]
impl Animation_Enumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, outer: crate::unity_engine::animation::Animation) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "unity_engine-animation")]
impl Animation_Enumerator {
    pub fn new(outer: crate::unity_engine::animation::Animation) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Animation_Enumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimation_EnumeratorMethods>::ctor(this, outer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animation/Animation.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Animation")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Animation {}

#[cfg(feature = "unity_engine-animation")]
#[::unity2::methods]
impl Animation {
    #[method(name = "get_clip", args = 0)]
    pub fn get_clip(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_clip", args = 1)]
    pub fn set_clip(self, value: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(name = "get_playAutomatically", args = 0)]
    pub fn get_play_automatically(self) -> bool;

    #[method(name = "set_playAutomatically", args = 1)]
    pub fn set_play_automatically(self, value: bool) -> ();

    #[method(name = "get_wrapMode", args = 0)]
    pub fn get_wrap_mode(self) -> crate::unity_engine::wrapmode::WrapMode;

    #[method(name = "set_wrapMode", args = 1)]
    pub fn set_wrap_mode(self, value: crate::unity_engine::wrapmode::WrapMode) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Stop", args = 1)]
    pub fn stop_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopNamed", args = 1)]
    pub fn stop_named(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Rewind", args = 0)]
    pub fn rewind(self) -> ();

    #[method(name = "Rewind", args = 1)]
    pub fn rewind_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "RewindNamed", args = 1)]
    pub fn rewind_named(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Sample", args = 0)]
    pub fn sample(self) -> ();

    #[method(name = "get_isPlaying", args = 0)]
    pub fn get_is_playing(self) -> bool;

    #[method(name = "IsPlaying", args = 1)]
    pub fn is_playing(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> bool;

    #[method(name = "Play", args = 1)]
    pub fn play_2(self, mode: crate::unity_engine::playmode::PlayMode) -> bool;

    #[method(name = "PlayDefaultAnimation", args = 1)]
    pub fn play_default_animation(self, mode: crate::unity_engine::playmode::PlayMode) -> bool;

    #[method(name = "Play", args = 1)]
    pub fn play_3(self, animation: ::unity2::Il2CppString) -> bool;

    #[method(name = "Play", args = 2)]
    pub fn play_4(
        self,
        animation: ::unity2::Il2CppString,
        mode: crate::unity_engine::playmode::PlayMode,
    ) -> bool;

    #[method(name = "CrossFade", args = 1)]
    pub fn cross_fade(self, animation: ::unity2::Il2CppString) -> ();

    #[method(name = "CrossFade", args = 2)]
    pub fn cross_fade_2(self, animation: ::unity2::Il2CppString, fade_length: f32) -> ();

    #[method(name = "CrossFade", args = 3)]
    pub fn cross_fade_3(
        self,
        animation: ::unity2::Il2CppString,
        fade_length: f32,
        mode: crate::unity_engine::playmode::PlayMode,
    ) -> ();

    #[method(name = "Blend", args = 1)]
    pub fn blend(self, animation: ::unity2::Il2CppString) -> ();

    #[method(name = "Blend", args = 2)]
    pub fn blend_2(self, animation: ::unity2::Il2CppString, target_weight: f32) -> ();

    #[method(name = "Blend", args = 3)]
    pub fn blend_3(
        self,
        animation: ::unity2::Il2CppString,
        target_weight: f32,
        fade_length: f32,
    ) -> ();

    #[method(name = "CrossFadeQueued", args = 1)]
    pub fn cross_fade_queued(
        self,
        animation: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "CrossFadeQueued", args = 2)]
    pub fn cross_fade_queued_2(
        self,
        animation: ::unity2::Il2CppString,
        fade_length: f32,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "CrossFadeQueued", args = 3)]
    pub fn cross_fade_queued_3(
        self,
        animation: ::unity2::Il2CppString,
        fade_length: f32,
        queue: crate::unity_engine::queuemode::QueueMode,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "CrossFadeQueued", args = 4)]
    pub fn cross_fade_queued_4(
        self,
        animation: ::unity2::Il2CppString,
        fade_length: f32,
        queue: crate::unity_engine::queuemode::QueueMode,
        mode: crate::unity_engine::playmode::PlayMode,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "PlayQueued", args = 1)]
    pub fn play_queued(
        self,
        animation: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "PlayQueued", args = 2)]
    pub fn play_queued_2(
        self,
        animation: ::unity2::Il2CppString,
        queue: crate::unity_engine::queuemode::QueueMode,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "PlayQueued", args = 3)]
    pub fn play_queued_3(
        self,
        animation: ::unity2::Il2CppString,
        queue: crate::unity_engine::queuemode::QueueMode,
        mode: crate::unity_engine::playmode::PlayMode,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "AddClip", args = 2)]
    pub fn add_clip(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
        new_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddClip", args = 4)]
    pub fn add_clip_2(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
        new_name: ::unity2::Il2CppString,
        first_frame: i32,
        last_frame: i32,
    ) -> ();

    #[method(name = "AddClip", args = 5)]
    pub fn add_clip_3(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
        new_name: ::unity2::Il2CppString,
        first_frame: i32,
        last_frame: i32,
        add_loop_frame: bool,
    ) -> ();

    #[method(name = "RemoveClip", args = 1)]
    pub fn remove_clip(self, clip: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(name = "RemoveClip", args = 1)]
    pub fn remove_clip_2(self, clip_name: ::unity2::Il2CppString) -> ();

    #[method(name = "RemoveClipNamed", args = 1)]
    pub fn remove_clip_named(self, clip_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetClipCount", args = 0)]
    pub fn get_clip_count(self) -> i32;

    #[method(name = "Play", args = 1)]
    pub fn play_5(self, mode: crate::unity_engine::animationplaymode::AnimationPlayMode) -> bool;

    #[method(name = "Play", args = 2)]
    pub fn play_6(
        self,
        animation: ::unity2::Il2CppString,
        mode: crate::unity_engine::animationplaymode::AnimationPlayMode,
    ) -> bool;

    #[method(name = "SyncLayer", args = 1)]
    pub fn sync_layer(self, layer: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetState", args = 1)]
    pub fn get_state(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "GetStateAtIndex", args = 1)]
    pub fn get_state_at_index(
        self,
        index: i32,
    ) -> crate::unity_engine::animationstate::AnimationState;

    #[method(name = "GetStateCount", args = 0)]
    pub fn get_state_count(self) -> i32;

    #[method(name = "GetClip", args = 1)]
    pub fn get_clip_2(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "get_animatePhysics", args = 0)]
    pub fn get_animate_physics(self) -> bool;

    #[method(name = "set_animatePhysics", args = 1)]
    pub fn set_animate_physics(self, value: bool) -> ();

    #[method(name = "get_animateOnlyIfVisible", args = 0)]
    pub fn get_animate_only_if_visible(self) -> bool;

    #[method(name = "set_animateOnlyIfVisible", args = 1)]
    pub fn set_animate_only_if_visible(self, value: bool) -> ();

    #[method(name = "get_cullingType", args = 0)]
    pub fn get_culling_type(
        self,
    ) -> crate::unity_engine::animationcullingtype::AnimationCullingType;

    #[method(name = "set_cullingType", args = 1)]
    pub fn set_culling_type(
        self,
        value: crate::unity_engine::animationcullingtype::AnimationCullingType,
    ) -> ();

    #[method(name = "get_localBounds", args = 0)]
    pub fn get_local_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "set_localBounds", args = 1)]
    pub fn set_local_bounds(self, value: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_localBounds_Injected", args = 1)]
    pub fn get_local_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "set_localBounds_Injected", args = 1)]
    pub fn set_local_bounds_injected(self, value: crate::unity_engine::bounds::Bounds) -> ();
}

#[cfg(feature = "unity_engine-animation")]
impl Animation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Animation),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimationMethods>::ctor(this);
        this
    }
}
