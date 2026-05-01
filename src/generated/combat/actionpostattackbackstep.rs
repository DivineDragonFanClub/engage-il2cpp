
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionpostattackbackstep/ActionPostAttackBackstep.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionPostAttackBackstep")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionPostAttackBackstep {}

#[cfg(feature = "combat-actionpostattackbackstep")]
#[::unity2::methods]
impl ActionPostAttackBackstep {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionpostattackbackstep")]
impl ActionPostAttackBackstep {
    pub fn new(
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionPostAttackBackstep),
                ::core::stringify!(new),
            )
        });
        <Self as IActionPostAttackBackstepMethods>::ctor(this, chr, phase);
        this
    }
}
