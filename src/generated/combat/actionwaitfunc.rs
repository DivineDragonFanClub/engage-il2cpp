
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionwaitfunc/ActionWaitFunc.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionWaitFunc")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionWaitFunc {
    #[rename(name = "m_Func")]
    pub m_func: crate::system::func_2::Func_2<crate::combat::character::Character, bool>,
}

#[cfg(feature = "combat-actionwaitfunc")]
#[::unity2::methods]
impl ActionWaitFunc {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        wait_func_while_true: crate::system::func_2::Func_2<
            crate::combat::character::Character,
            bool,
        >,
    ) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionwaitfunc")]
impl ActionWaitFunc {
    pub fn new(
        chr: crate::combat::character::Character,
        wait_func_while_true: crate::system::func_2::Func_2<
            crate::combat::character::Character,
            bool,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionWaitFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IActionWaitFuncMethods>::ctor(this, chr, wait_func_while_true);
        this
    }
}
