
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaloutsidemenuitem/AnimalOutsideMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AnimalOutsideMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AnimalOutsideMenuItem {
    #[rename(name = "m_pid")]
    pub m_pid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-animaloutsidemenuitem")]
#[::unity2::methods]
impl AnimalOutsideMenuItem {
    #[method(name = "get_Animal", args = 0)]
    pub fn get_animal(self) -> crate::app::animaldata::AnimalData;

    #[method(name = "set_Animal", args = 1)]
    pub fn set_animal(self, value: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "get_IsLastMenu", args = 0)]
    pub fn get_is_last_menu(self) -> bool;

    #[method(name = "set_IsLastMenu", args = 1)]
    pub fn set_is_last_menu(self, value: bool) -> ();

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "set_IsActive", args = 1)]
    pub fn set_is_active(self, value: bool) -> ();

    #[method(name = "get_IsActiveSelect", args = 0)]
    pub fn get_is_active_select(self) -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, pid: ::unity2::Il2CppString, last_menu: bool) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animaloutsidemenuitem")]
impl AnimalOutsideMenuItem {
    pub fn new(pid: ::unity2::Il2CppString, last_menu: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalOutsideMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalOutsideMenuItemMethods>::ctor(this, pid, last_menu);
        this
    }
}
