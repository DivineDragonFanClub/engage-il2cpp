
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionvoice/ActionVoice.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionVoice")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionVoice {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-actionvoice")]
#[::unity2::methods]
impl ActionVoice {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, chr: crate::combat::character::Character, name: ::unity2::Il2CppString)
        -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();
}

#[cfg(feature = "combat-actionvoice")]
impl ActionVoice {
    pub fn new(chr: crate::combat::character::Character, name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionVoice),
                ::core::stringify!(new),
            )
        });
        <Self as IActionVoiceMethods>::ctor(this, chr, name);
        this
    }
}
