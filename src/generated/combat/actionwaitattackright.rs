
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionwaitattackright/ActionWaitAttackRight.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionWaitAttackRight")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionWaitAttackRight {}

#[cfg(feature = "combat-actionwaitattackright")]
#[::unity2::methods]
impl ActionWaitAttackRight {
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

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();
}

#[cfg(feature = "combat-actionwaitattackright")]
impl ActionWaitAttackRight {
    pub fn new(
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionWaitAttackRight),
                ::core::stringify!(new),
            )
        });
        <Self as IActionWaitAttackRightMethods>::ctor(this, chr, phase);
        this
    }
}
