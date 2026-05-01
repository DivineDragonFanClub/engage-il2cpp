
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiontalk/ActionTalk.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionTalk")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionTalk {
    #[rename(name = "isEnd")]
    pub is_end: bool,
    #[rename(name = "calc")]
    pub calc: crate::app::battlecalculator::BattleCalculator,
}

#[cfg(feature = "combat-actiontalk")]
#[::unity2::methods]
impl ActionTalk {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        calc: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();
}

#[cfg(feature = "combat-actiontalk")]
impl ActionTalk {
    pub fn new(
        chr: crate::combat::character::Character,
        calc: crate::app::battlecalculator::BattleCalculator,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionTalk),
                ::core::stringify!(new),
            )
        });
        <Self as IActionTalkMethods>::ctor(this, chr, calc);
        this
    }
}
