
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactertimespace/CharacterTimespace.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterTimespace")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterTimespace {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "DefaultPlayBackRate")]
    pub default_play_back_rate: f32,
    #[rename(name = "m_bImpactAdjustReserved")]
    pub m_b_impact_adjust_reserved: bool,
    #[rename(name = "m_bImpactAdjust命中信号実行")]
    pub m_b_impact_adjust______: bool,
    #[rename(name = "m_ImpactAdjustWorldTime")]
    pub m_impact_adjust_world_time: f32,
    #[rename(name = "IsStartCalled")]
    pub is_start_called: bool,
}

#[cfg(feature = "combat-charactertimespace")]
#[::unity2::methods]
impl CharacterTimespace {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_PlaybackRate", args = 0)]
    pub fn get_playback_rate(self) -> f32;

    #[method(name = "set_PlaybackRate", args = 1)]
    pub fn set_playback_rate(self, value: f32) -> ();

    #[method(name = "get_DeltaTime", args = 0)]
    pub fn get_delta_time(self) -> f32;

    #[method(name = "get_SmoothDeltaTime", args = 0)]
    pub fn get_smooth_delta_time(self) -> f32;

    #[method(name = "get_PerAnimationPlaybackRate", args = 0)]
    pub fn get_per_animation_playback_rate(self) -> f32;

    #[method(name = "set_PerAnimationPlaybackRate", args = 1)]
    pub fn set_per_animation_playback_rate(self, value: f32) -> ();

    #[method(name = "get_SignalWorldTimeScale", args = 0)]
    pub fn get_signal_world_time_scale(self) -> f32;

    #[method(name = "set_SignalWorldTimeScale", args = 1)]
    pub fn set_signal_world_time_scale(self, value: f32) -> ();

    #[method(name = "get_MotionSpeed", args = 0)]
    pub fn get_motion_speed(self) -> f32;

    #[method(name = "ResetPerPlay", args = 0)]
    pub fn reset_per_play(self) -> ();

    #[method(name = "SetImpactAdjust", args = 1)]
    pub fn set_impact_adjust(self, _unnamed: bool) -> ();

    #[method(name = "SetImpactAdjust", args = 1)]
    pub fn set_impact_adjust_2(self, time: f32) -> ();

    #[method(name = "RegisterSignalObservers", args = 0)]
    pub fn register_signal_observers(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "CalcTimespanInRealtime", args = 5)]
    pub fn calc_timespan_in_realtime(
        events: ::unity2::Array<crate::unity_engine::animationevent::AnimationEvent>,
        heaviness_rate: f32,
        agility_rate: f32,
        start_time: f32,
        end_time: f32,
    ) -> f32;

    #[method(name = "CalcTimespanInRealtime", args = 2)]
    pub fn calc_timespan_in_realtime_2(self, start_time: f32, end_time: f32) -> f32;

    #[method(name = "CalcTimespanInRealtime", args = 3)]
    pub fn calc_timespan_in_realtime_3(
        self,
        store: crate::combat::prefetchedsignal::PrefetchedSignal,
        start_time: f32,
        end_time: f32,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactertimespace")]
impl CharacterTimespace {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterTimespace),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterTimespaceMethods>::ctor(this);
        this
    }
}
