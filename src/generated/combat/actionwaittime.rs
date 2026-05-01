
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionwaittime/ActionWaitTime.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionWaitTime")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionWaitTime {
    #[rename(name = "m_Seconds")]
    pub m_seconds: f32,
}

#[cfg(feature = "combat-actionwaittime")]
#[::unity2::methods]
impl ActionWaitTime {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, chr: crate::combat::character::Character, seconds: f32) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionwaittime")]
impl ActionWaitTime {
    pub fn new(chr: crate::combat::character::Character, seconds: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionWaitTime),
                ::core::stringify!(new),
            )
        });
        <Self as IActionWaitTimeMethods>::ctor(this, chr, seconds);
        this
    }
}
