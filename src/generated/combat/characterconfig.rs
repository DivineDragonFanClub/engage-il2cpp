
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterconfig/CharacterConfig.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterConfig")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterConfig {
    #[rename(name = "InitialDebugPlaybackRate")]
    pub initial_debug_playback_rate: f32,
    #[rename(name = "InitialBackwardCancelRatio")]
    pub initial_backward_cancel_ratio: f32,
    #[rename(name = "InitialHeavinessFactor")]
    pub initial_heaviness_factor: f32,
    #[rename(name = "InitialAgilityFactor")]
    pub initial_agility_factor: f32,
    #[rename(name = "InitialHitRatio")]
    pub initial_hit_ratio: f32,
    #[rename(name = "InitialCriticalRatio")]
    pub initial_critical_ratio: f32,
    #[rename(name = "InitialGuardRatio")]
    pub initial_guard_ratio: f32,
    #[rename(name = "InitialDamageRatio")]
    pub initial_damage_ratio: f32,
    #[rename(name = "InitialAttackRatio")]
    pub initial_attack_ratio: f32,
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
}

#[cfg(feature = "combat-characterconfig")]
#[::unity2::methods]
impl CharacterConfig {
    #[method(name = "set_DebugPlaybackRate", args = 1)]
    pub fn set_debug_playback_rate(self, value: f32) -> ();

    #[method(name = "get_DebugPlaybackRate", args = 0)]
    pub fn get_debug_playback_rate(self) -> f32;

    #[method(name = "set_BackwardCancelRatio", args = 1)]
    pub fn set_backward_cancel_ratio(self, value: f32) -> ();

    #[method(name = "get_BackwardCancelRatio", args = 0)]
    pub fn get_backward_cancel_ratio(self) -> f32;

    #[method(name = "set_HeavinessFactor", args = 1)]
    pub fn set_heaviness_factor(self, value: f32) -> ();

    #[method(name = "get_HeavinessFactor", args = 0)]
    pub fn get_heaviness_factor(self) -> f32;

    #[method(name = "set_AgilityFactor", args = 1)]
    pub fn set_agility_factor(self, value: f32) -> ();

    #[method(name = "get_AgilityFactor", args = 0)]
    pub fn get_agility_factor(self) -> f32;

    #[method(name = "set_HitRatio", args = 1)]
    pub fn set_hit_ratio(self, value: f32) -> ();

    #[method(name = "get_HitRatio", args = 0)]
    pub fn get_hit_ratio(self) -> f32;

    #[method(name = "set_CriticalRatio", args = 1)]
    pub fn set_critical_ratio(self, value: f32) -> ();

    #[method(name = "get_CriticalRatio", args = 0)]
    pub fn get_critical_ratio(self) -> f32;

    #[method(name = "set_GuardRatio", args = 1)]
    pub fn set_guard_ratio(self, value: f32) -> ();

    #[method(name = "get_GuardRatio", args = 0)]
    pub fn get_guard_ratio(self) -> f32;

    #[method(name = "set_DamageRatio", args = 1)]
    pub fn set_damage_ratio(self, value: f32) -> ();

    #[method(name = "get_DamageRatio", args = 0)]
    pub fn get_damage_ratio(self) -> f32;

    #[method(name = "set_AttackRatio", args = 1)]
    pub fn set_attack_ratio(self, value: f32) -> ();

    #[method(name = "get_AttackRatio", args = 0)]
    pub fn get_attack_ratio(self) -> f32;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "StartImpl", args = 0)]
    pub fn start_impl(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "InternalGUI", args = 0)]
    pub fn internal_gui(self) -> ();

    #[method(name = "GetAttackSideByDice", args = 0)]
    pub fn get_attack_side_by_dice(self) -> i32;

    #[method(name = "MakePhaseD", args = 0)]
    pub fn make_phase_d(self) -> crate::combat::phase::Phase;

    #[method(name = "IN", args = 1)]
    pub fn r#in(value: f32) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterconfig")]
impl CharacterConfig {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterConfig),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterConfigMethods>::ctor(this);
        this
    }
}
