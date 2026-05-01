
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actioninvoke/ActionInvoke.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionInvoke")]
#[parent(crate::combat::state::State)]
pub struct ActionInvoke {
    #[rename(name = "m_Func")]
    pub m_func: crate::system::action::Action,
}

#[cfg(feature = "combat-actioninvoke")]
#[::unity2::methods]
impl ActionInvoke {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, func: crate::system::action::Action) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();
}

#[cfg(feature = "combat-actioninvoke")]
impl ActionInvoke {
    pub fn new(func: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionInvoke),
                ::core::stringify!(new),
            )
        });
        <Self as IActionInvokeMethods>::ctor(this, func);
        this
    }
}
