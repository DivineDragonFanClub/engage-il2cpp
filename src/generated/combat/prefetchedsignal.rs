
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/prefetchedsignal/PrefetchedSignal.md")))]
#[::unity2::class(namespace = "Combat", name = "PrefetchedSignal")]
#[parent(crate::system::object::Object)]
pub struct PrefetchedSignal {
    #[static_field]
    #[rename(name = "s_null")]
    pub s_null: crate::combat::prefetchedsignal::PrefetchedSignal,
    #[rename(name = "m_ClipEvents")]
    pub m_clip_events: ::unity2::Array<crate::unity_engine::animationevent::AnimationEvent>,
    #[rename(name = "backwardCancelA")]
    pub backward_cancel_a: f32,
    #[rename(name = "backwardCancelB")]
    pub backward_cancel_b: f32,
    #[rename(name = "HitTimesNT")]
    pub hit_times_nt: crate::system::collections::generic::list_1::List_1<f32>,
    #[rename(name = "HitSignals")]
    pub hit_signals: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::animationevent::AnimationEvent,
    >,
    #[rename(name = "localDieFallPos")]
    pub local_die_fall_pos: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "combat-prefetchedsignal")]
#[::unity2::methods]
impl PrefetchedSignal {
    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(a: crate::combat::prefetchedsignal::PrefetchedSignal) -> bool;

    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "CreateNull", args = 0)]
    pub fn create_null() -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "get_StateName", args = 0)]
    pub fn get_state_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StateName", args = 1)]
    pub fn set_state_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_StateHash", args = 0)]
    pub fn get_state_hash(self) -> i32;

    #[method(name = "set_StateHash", args = 1)]
    pub fn set_state_hash(self, value: i32) -> ();

    #[method(name = "get_isInternalNull", args = 0)]
    pub fn get_is_internal_null(self) -> bool;

    #[method(name = "set_isInternalNull", args = 1)]
    pub fn set_is_internal_null(self, value: bool) -> ();

    #[method(name = "get_Clip", args = 0)]
    pub fn get_clip(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_Clip", args = 1)]
    pub fn set_clip(self, value: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(name = "get_Curves", args = 0)]
    pub fn get_curves(self) -> crate::combat::prefetchedcurve::PrefetchedCurve;

    #[method(name = "set_Curves", args = 1)]
    pub fn set_curves(self, value: crate::combat::prefetchedcurve::PrefetchedCurve) -> ();

    #[method(name = "get_ClipLength", args = 0)]
    pub fn get_clip_length(self) -> f32;

    #[method(name = "set_ClipLength", args = 1)]
    pub fn set_clip_length(self, value: f32) -> ();

    #[method(name = "get_FT2NT", args = 0)]
    pub fn get_ft2nt(self) -> f32;

    #[method(name = "set_FT2NT", args = 1)]
    pub fn set_ft2nt(self, value: f32) -> ();

    #[method(name = "get_AttackFarRange", args = 0)]
    pub fn get_attack_far_range(self) -> f32;

    #[method(name = "set_AttackFarRange", args = 1)]
    pub fn set_attack_far_range(self, value: f32) -> ();

    #[method(name = "get_AttackNearRange", args = 0)]
    pub fn get_attack_near_range(self) -> f32;

    #[method(name = "set_AttackNearRange", args = 1)]
    pub fn set_attack_near_range(self, value: f32) -> ();

    #[method(name = "get_ForwardCancel", args = 0)]
    pub fn get_forward_cancel(self) -> f32;

    #[method(name = "set_ForwardCancel", args = 1)]
    pub fn set_forward_cancel(self, value: f32) -> ();

    #[method(name = "get_RushForwardCancel", args = 0)]
    pub fn get_rush_forward_cancel(self) -> f32;

    #[method(name = "set_RushForwardCancel", args = 1)]
    pub fn set_rush_forward_cancel(self, value: f32) -> ();

    #[method(name = "BackwardCancel", args = 1)]
    pub fn backward_cancel(self, ratio: f32) -> f32;

    #[method(name = "get_TransitionDuration", args = 0)]
    pub fn get_transition_duration(self) -> f32;

    #[method(name = "set_TransitionDuration", args = 1)]
    pub fn set_transition_duration(self, value: f32) -> ();

    #[method(name = "get_LocalRootMoved", args = 0)]
    pub fn get_local_root_moved(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LocalRootMoved", args = 1)]
    pub fn set_local_root_moved(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_WeaponStyle", args = 0)]
    pub fn get_weapon_style(self) -> crate::combat::weaponstyle::WeaponStyle;

    #[method(name = "set_WeaponStyle", args = 1)]
    pub fn set_weapon_style(self, value: crate::combat::weaponstyle::WeaponStyle) -> ();

    #[method(name = "get_HitSignal", args = 0)]
    pub fn get_hit_signal(self) -> crate::unity_engine::animationevent::AnimationEvent;

    #[method(name = "set_HitSignal", args = 1)]
    pub fn set_hit_signal(self, value: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "get_HitCount", args = 0)]
    pub fn get_hit_count(self) -> i32;

    #[method(name = "set_HitCount", args = 1)]
    pub fn set_hit_count(self, value: i32) -> ();

    #[method(name = "get_HitTime", args = 0)]
    pub fn get_hit_time(self) -> f32;

    #[method(name = "set_HitTime", args = 1)]
    pub fn set_hit_time(self, value: f32) -> ();

    #[method(name = "get_LocalHitPos", args = 0)]
    pub fn get_local_hit_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LocalHitPos", args = 1)]
    pub fn set_local_hit_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_LocalHitDir", args = 0)]
    pub fn get_local_hit_dir(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LocalHitDir", args = 1)]
    pub fn set_local_hit_dir(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_IsKnockable", args = 0)]
    pub fn get_is_knockable(self) -> bool;

    #[method(name = "get_ShootTime", args = 0)]
    pub fn get_shoot_time(self) -> f32;

    #[method(name = "set_ShootTime", args = 1)]
    pub fn set_shoot_time(self, value: f32) -> ();

    #[method(name = "get_LocalShootPos", args = 0)]
    pub fn get_local_shoot_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LocalShootPos", args = 1)]
    pub fn set_local_shoot_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_LocalShootDir", args = 0)]
    pub fn get_local_shoot_dir(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LocalShootDir", args = 1)]
    pub fn set_local_shoot_dir(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_HitOrShootTime", args = 0)]
    pub fn get_hit_or_shoot_time(self) -> f32;

    #[method(name = "get_HumanDieFallTime", args = 0)]
    pub fn get_human_die_fall_time(self) -> f32;

    #[method(name = "set_HumanDieFallTime", args = 1)]
    pub fn set_human_die_fall_time(self, value: f32) -> ();

    #[method(name = "get_MotionMatchingVelocity", args = 0)]
    pub fn get_motion_matching_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_MotionMatchingVelocity", args = 1)]
    pub fn set_motion_matching_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_MotionMatchingTorque", args = 0)]
    pub fn get_motion_matching_torque(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_MotionMatchingTorque", args = 1)]
    pub fn set_motion_matching_torque(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> f32;

    #[method(name = "set_Score", args = 1)]
    pub fn set_score(self, value: f32) -> ();

    #[method(name = "get_IsAttackRangeValid", args = 0)]
    pub fn get_is_attack_range_valid(self) -> bool;

    #[method(name = "get_ForwardCancelNT", args = 0)]
    pub fn get_forward_cancel_nt(self) -> f32;

    #[method(name = "get_RushForwardCancelNT", args = 0)]
    pub fn get_rush_forward_cancel_nt(self) -> f32;

    #[method(name = "BackwardCancelNT", args = 1)]
    pub fn backward_cancel_nt(self, ratio: f32) -> f32;

    #[method(name = "GetBackwardCancelABNTForDebug", args = 2)]
    pub fn get_backward_cancel_abnt_for_debug(self, a: f32, b: f32) -> ();

    #[method(name = "WorldRootMoved", args = 1)]
    pub fn world_root_moved(
        self,
        t: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "WorldDieFallPos", args = 2)]
    pub fn world_die_fall_pos(
        self,
        t: crate::unity_engine::transform::Transform,
        index: i32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "WillCollide", args = 1)]
    pub fn will_collide(self, cp: crate::combat::character::Character) -> bool;

    #[method(name = "CalcDistanceToColliderAtRootMoved", args = 1)]
    pub fn calc_distance_to_collider_at_root_moved(
        self,
        cp: crate::combat::character::Character,
    ) -> f32;

    #[method(name = "IsObstractedBetweenRootMovedAndEnemy", args = 1)]
    pub fn is_obstracted_between_root_moved_and_enemy(
        self,
        cp: crate::combat::character::Character,
    ) -> bool;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        state_name: ::unity2::Il2CppString,
        state_hash: i32,
        body_go: crate::unity_engine::gameobject::GameObject,
        body_anim: crate::unity_engine::animationclip::AnimationClip,
        ride_go: crate::unity_engine::gameobject::GameObject,
        ride_anim: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "GetPrefetchedCurve", args = 1)]
    pub fn get_prefetched_curve(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::combat::prefetchedcurve::PrefetchedCurve;

    #[method(name = "ImportFromSignals", args = 1)]
    pub fn import_from_signals(
        self,
        events: ::unity2::Array<crate::unity_engine::animationevent::AnimationEvent>,
    ) -> ();
}

#[cfg(feature = "combat-prefetchedsignal")]
impl PrefetchedSignal {
    pub fn new(
        state_name: ::unity2::Il2CppString,
        state_hash: i32,
        body_go: crate::unity_engine::gameobject::GameObject,
        body_anim: crate::unity_engine::animationclip::AnimationClip,
        ride_go: crate::unity_engine::gameobject::GameObject,
        ride_anim: crate::unity_engine::animationclip::AnimationClip,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PrefetchedSignal),
                ::core::stringify!(new),
            )
        });
        <Self as IPrefetchedSignalMethods>::ctor(
            this, state_name, state_hash, body_go, body_anim, ride_go, ride_anim,
        );
        this
    }
}
