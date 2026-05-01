
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/anibodyride/AniBodyRide.md")))]
#[::unity2::class(namespace = "Combat", name = "AniBodyRide")]
#[parent(crate::system::object::Object)]
pub struct AniBodyRide {
    #[rename(name = "initialized")]
    pub initialized: bool,
    #[rename(name = "playingIndex")]
    pub playing_index: i32,
}

#[cfg(feature = "combat-anibodyride")]
#[::unity2::methods]
impl AniBodyRide {
    #[method(name = "get_gameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_gameObject", args = 1)]
    pub fn set_game_object(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_animator", args = 0)]
    pub fn get_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "set_animator", args = 1)]
    pub fn set_animator(self, value: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "get_clips", args = 0)]
    pub fn get_clips(self) -> ::unity2::Array<crate::unity_engine::animationclip::AnimationClip>;

    #[method(name = "set_clips", args = 1)]
    pub fn set_clips(
        self,
        value: ::unity2::Array<crate::unity_engine::animationclip::AnimationClip>,
    ) -> ();

    #[method(name = "get_clipNames", args = 0)]
    pub fn get_clip_names(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_clipNames", args = 1)]
    pub fn set_clip_names(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Playing", args = 0)]
    pub fn get_playing(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "get_PlayingClipLength", args = 0)]
    pub fn get_playing_clip_length(self) -> f32;

    #[method(name = "get_IsValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "get_IsPlaying", args = 0)]
    pub fn get_is_playing(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate(self, time: f32) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play(self, index: i32) -> ();

    #[method(name = "FindIndex", args = 1)]
    pub fn find_index(self, name: ::unity2::Il2CppString) -> i32;
}

#[cfg(feature = "combat-anibodyride")]
impl AniBodyRide {
    pub fn new(go: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AniBodyRide),
                ::core::stringify!(new),
            )
        });
        <Self as IAniBodyRideMethods>::ctor(this, go);
        this
    }
}
