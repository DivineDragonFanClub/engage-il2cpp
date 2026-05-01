
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiondamage/ActionDamage.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionDamage")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionDamage {
    #[rename(name = "m_Attacker")]
    pub m_attacker: crate::combat::character::Character,
    #[rename(name = "m_Damager")]
    pub m_damager: crate::combat::character::Character,
    #[rename(name = "m_HitEv")]
    pub m_hit_ev: crate::unity_engine::animationevent::AnimationEvent,
}

#[cfg(feature = "combat-actiondamage")]
#[::unity2::methods]
impl ActionDamage {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_AbortByInterrupt", args = 0)]
    pub fn get_abort_by_interrupt(self) -> bool;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        attacker: crate::combat::character::Character,
        damager: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> ();

    #[method(name = "RunLyingWhenDie", args = 0)]
    pub fn run_lying_when_die(self) -> ();

    #[method(name = "ResolveAll", args = 2)]
    pub fn resolve_all(
        phase: crate::combat::phase::Phase,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> i32;

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actiondamage")]
impl ActionDamage {
    pub fn new(
        attacker: crate::combat::character::Character,
        damager: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionDamage),
                ::core::stringify!(new),
            )
        });
        <Self as IActionDamageMethods>::ctor(this, attacker, damager, phase, ev);
        this
    }
}
