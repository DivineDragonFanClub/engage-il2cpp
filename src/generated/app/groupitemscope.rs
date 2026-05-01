
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/groupitemscope/GroupItemScope.md")))]
#[::unity2::class(namespace = "App", name = "GroupItemScope")]
#[parent(crate::system::object::Object)]
pub struct GroupItemScope {
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::debugmenu::DebugMenu,
}

#[cfg(feature = "app-groupitemscope")]
#[::unity2::methods]
impl GroupItemScope {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu: crate::app::debugmenu::DebugMenu,
        name: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        menu: crate::app::debugmenu::DebugMenu,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> ();

    #[method(name = "Setup", args = 4)]
    pub fn setup(
        self,
        menu: crate::app::debugmenu::DebugMenu,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "app-groupitemscope")]
impl GroupItemScope {
    pub fn new(
        menu: crate::app::debugmenu::DebugMenu,
        name: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GroupItemScope),
                ::core::stringify!(new),
            )
        });
        <Self as IGroupItemScopeMethods>::ctor(this, menu, name, state);
        this
    }

    pub fn new_2(
        menu: crate::app::debugmenu::DebugMenu,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GroupItemScope),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGroupItemScopeMethods>::ctor_2(this, menu, name, english, state);
        this
    }
}
