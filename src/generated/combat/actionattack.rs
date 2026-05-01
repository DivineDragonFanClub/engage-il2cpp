
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::actiondisposerholder::ActionDisposerHolder;
use crate::combat::actiondisposerholder::IActionDisposerHolder;
use crate::combat::actionobservable::ActionObservable;
use crate::combat::actionobservable::IActionObservable;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionattack/ActionAttack.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionAttack")]
#[parent(crate::combat::actionobservable::ActionObservable)]
pub struct ActionAttack {
    #[rename(name = "m_ChainAttackTimeout")]
    pub m_chain_attack_timeout: f32,
    #[rename(name = "m_ChainSpeedRate")]
    pub m_chain_speed_rate: f32,
    #[rename(name = "m_Stage")]
    pub m_stage: crate::combat::actionattack::ActionAttack_Stage,
    #[rename(name = "m_WorldArrivalTime")]
    pub m_world_arrival_time: f32,
    #[rename(name = "m_FarAttackRangeSq")]
    pub m_far_attack_range_sq: f32,
    #[rename(name = "m_ChainGuard")]
    pub m_chain_guard: crate::combat::character::Character,
    #[rename(name = "m_bHitPassed")]
    pub m_b_hit_passed: bool,
    #[rename(name = "m_TimeToNext")]
    pub m_time_to_next: f32,
    #[rename(name = "m_IsNotRepelled")]
    pub m_is_not_repelled: bool,
    #[rename(name = "m_再生ジャンプPassed")]
    pub m_______passed: bool,
}

#[cfg(feature = "combat-actionattack")]
#[::unity2::methods]
impl ActionAttack {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        time_to_chain_attack: f32,
    ) -> ();

    #[method(name = "NextStage", args = 0)]
    pub fn next_stage(self) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "EnterApproach", args = 0)]
    pub fn enter_approach(self) -> ();

    #[method(name = "ChainGuardPresentation", args = 0)]
    pub fn chain_guard_presentation(self) -> ();

    #[method(name = "UpdateApproach", args = 0)]
    pub fn update_approach(self) -> ();

    #[method(name = "get_AbortByInterrupt", args = 0)]
    pub fn get_abort_by_interrupt(self) -> bool;

    #[method(name = "EnterAttack", args = 0)]
    pub fn enter_attack(self) -> ();

    #[method(name = "UpdateAttack", args = 0)]
    pub fn update_attack(self) -> ();

    #[method(name = "Miss", args = 1)]
    pub fn miss(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "Hit", args = 1)]
    pub fn hit(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "SelfHit", args = 1)]
    pub fn self_hit(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "Guard", args = 1)]
    pub fn guard(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();
}

#[cfg(feature = "combat-actionattack")]
impl ActionAttack {
    pub fn new(
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        time_to_chain_attack: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IActionAttackMethods>::ctor(this, chr, phase, time_to_chain_attack);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionattack/ActionAttack_Stage.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ActionAttack_Stage {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ActionAttack_Stage {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "ActionAttack.Stage";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ActionAttack_Stage {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ActionAttack_Stage {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn approach() -> Self {
        Self { value: 1 }
    }

    pub fn attack() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}
