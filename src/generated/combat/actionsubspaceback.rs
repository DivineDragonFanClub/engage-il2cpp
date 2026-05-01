
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionsubspaceback/ActionSubspaceBack.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionSubspaceBack")]
#[parent(crate::combat::state::State)]
pub struct ActionSubspaceBack {
    #[rename(name = "m_Side")]
    pub m_side: i32,
    #[rename(name = "m_IsLastBossDead")]
    pub m_is_last_boss_dead: bool,
}

#[cfg(feature = "combat-actionsubspaceback")]
#[::unity2::methods]
impl ActionSubspaceBack {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, phase: crate::combat::phase::Phase) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionsubspaceback")]
impl ActionSubspaceBack {
    pub fn new(phase: crate::combat::phase::Phase) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionSubspaceBack),
                ::core::stringify!(new),
            )
        });
        <Self as IActionSubspaceBackMethods>::ctor(this, phase);
        this
    }
}
