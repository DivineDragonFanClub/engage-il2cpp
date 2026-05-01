
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::stringitem::IStringItem;
use crate::app::stringitem::StringItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/groupbeginitem/GroupBeginItem.md")))]
#[::unity2::class(namespace = "App", name = "GroupBeginItem")]
#[parent(crate::app::stringitem::StringItem)]
pub struct GroupBeginItem {
    #[static_field]
    #[rename(name = "FontColor")]
    pub font_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-groupbeginitem")]
#[::unity2::methods]
impl GroupBeginItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::menuitem::MenuItem_Kind;

    #[method(name = "GetFontColor", args = 0)]
    pub fn get_font_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "LeftCall", args = 0)]
    pub fn left_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "RightCall", args = 0)]
    pub fn right_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "ChangeState", args = 1)]
    pub fn change_state(self, state: crate::app::menuitem::MenuItem_State) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-groupbeginitem")]
impl GroupBeginItem {
    pub fn new(name: ::unity2::Il2CppString, state: crate::app::menuitem::MenuItem_State) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GroupBeginItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGroupBeginItemMethods>::ctor(this, name, state);
        this
    }

    pub fn new_2(
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GroupBeginItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGroupBeginItemMethods>::ctor_2(this, name, english, state);
        this
    }
}
