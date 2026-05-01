
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/state/State.md")))]
#[::unity2::class(namespace = "Combat", name = "State")]
#[parent(crate::system::object::Object)]
pub struct State {
    #[rename(name = "progress")]
    pub progress: crate::combat::stateprogress::StateProgress,
}

#[cfg(feature = "combat-state")]
#[::unity2::methods]
impl State {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "OnLateUpdate", args = 0)]
    pub fn on_late_update(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "get_AbortByInterrupt", args = 0)]
    pub fn get_abort_by_interrupt(self) -> bool;
}

#[cfg(feature = "combat-state")]
impl State {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(State),
                ::core::stringify!(new),
            )
        });
        <Self as IStateMethods>::ctor(this);
        this
    }
}
