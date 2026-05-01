
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionplay/ActionPlay.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionPlay")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionPlay {
    #[rename(name = "m_Hash")]
    pub m_hash: i32,
    #[rename(name = "m_WaitIdle")]
    pub m_wait_idle: bool,
    #[rename(name = "m_StartTime")]
    pub m_start_time: f32,
}

#[cfg(feature = "combat-actionplay")]
#[::unity2::methods]
impl ActionPlay {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_AbortByInterrupt", args = 0)]
    pub fn get_abort_by_interrupt(self) -> bool;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        hash: i32,
        wait_idle: bool,
        start_time: f32,
    ) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionplay")]
impl ActionPlay {
    pub fn new(
        chr: crate::combat::character::Character,
        hash: i32,
        wait_idle: bool,
        start_time: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionPlay),
                ::core::stringify!(new),
            )
        });
        <Self as IActionPlayMethods>::ctor(this, chr, hash, wait_idle, start_time);
        this
    }
}
