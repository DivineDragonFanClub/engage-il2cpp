
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionbase/ActionBase.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionBase")]
#[parent(crate::combat::state::State)]
pub struct ActionBase {}

#[cfg(feature = "combat-actionbase")]
#[::unity2::methods]
impl ActionBase {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "set_CP", args = 1)]
    pub fn set_cp(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_m_Phase", args = 0)]
    pub fn get_m_phase(self) -> crate::combat::phase::Phase;

    #[method(name = "set_m_Phase", args = 1)]
    pub fn set_m_phase(self, value: crate::combat::phase::Phase) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> ();

    #[method(name = "SetPhaseForRush", args = 1)]
    pub fn set_phase_for_rush(self, phase: crate::combat::phase::Phase) -> ();
}

#[cfg(feature = "combat-actionbase")]
impl ActionBase {
    pub fn new(
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionBase),
                ::core::stringify!(new),
            )
        });
        <Self as IActionBaseMethods>::ctor(this, chr, phase);
        this
    }
}
