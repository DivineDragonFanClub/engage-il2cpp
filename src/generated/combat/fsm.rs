
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsm/FSM.md")))]
#[::unity2::class(namespace = "Combat", name = "FSM")]
#[parent(crate::system::object::Object)]
pub struct FSM {
    #[rename(name = "StateList")]
    pub state_list: crate::system::collections::generic::linkedlist_1::LinkedList_1<
        crate::combat::state::State,
    >,
}

#[cfg(feature = "combat-fsm")]
#[::unity2::methods]
impl FSM {
    #[method(name = "get_CurrentState", args = 0)]
    pub fn get_current_state(self) -> crate::combat::state::State;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "UpdateCore", args = 1)]
    pub fn update_core(
        self,
        update_func: crate::system::action_1::Action_1<crate::combat::state::State>,
    ) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, state: crate::combat::state::State) -> ();

    #[method(name = "AddFirst", args = 1)]
    pub fn add_first(self, state: crate::combat::state::State) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_DebuggerDisplay", args = 0)]
    pub fn get_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "combat-fsm")]
impl FSM {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSM),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMMethods>::ctor(this);
        this
    }
}
