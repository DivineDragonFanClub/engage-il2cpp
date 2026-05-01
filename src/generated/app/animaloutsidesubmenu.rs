
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaloutsidesubmenu/AnimalOutsideSubMenu_RemoveMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AnimalOutsideSubMenu.RemoveMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AnimalOutsideSubMenu_RemoveMenuItem {}

#[cfg(feature = "app-animaloutsidesubmenu")]
#[::unity2::methods]
impl AnimalOutsideSubMenu_RemoveMenuItem {
    #[method(name = "get_ParentMenuItem", args = 0)]
    pub fn get_parent_menu_item(self) -> crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem;

    #[method(name = "set_ParentMenuItem", args = 1)]
    pub fn set_parent_menu_item(
        self,
        value: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        parent_menu_item: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animaloutsidesubmenu")]
impl AnimalOutsideSubMenu_RemoveMenuItem {
    pub fn new(parent_menu_item: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalOutsideSubMenu_RemoveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalOutsideSubMenu_RemoveMenuItemMethods>::ctor(this, parent_menu_item);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaloutsidesubmenu/AnimalOutsideSubMenu.md")))]
#[::unity2::class(namespace = "App", name = "AnimalOutsideSubMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct AnimalOutsideSubMenu {}

#[cfg(feature = "app-animaloutsidesubmenu")]
#[::unity2::methods]
impl AnimalOutsideSubMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        parent_menu_item: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animaloutsidesubmenu")]
impl AnimalOutsideSubMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalOutsideSubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalOutsideSubMenuMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaloutsidesubmenu/AnimalOutsideSubMenu_ExchangeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AnimalOutsideSubMenu.ExchangeMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AnimalOutsideSubMenu_ExchangeMenuItem {}

#[cfg(feature = "app-animaloutsidesubmenu")]
#[::unity2::methods]
impl AnimalOutsideSubMenu_ExchangeMenuItem {
    #[method(name = "get_ParentMenuItem", args = 0)]
    pub fn get_parent_menu_item(self) -> crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem;

    #[method(name = "set_ParentMenuItem", args = 1)]
    pub fn set_parent_menu_item(
        self,
        value: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        parent_menu_item: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animaloutsidesubmenu")]
impl AnimalOutsideSubMenu_ExchangeMenuItem {
    pub fn new(parent_menu_item: crate::app::animaloutsidemenuitem::AnimalOutsideMenuItem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalOutsideSubMenu_ExchangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalOutsideSubMenu_ExchangeMenuItemMethods>::ctor(this, parent_menu_item);
        this
    }
}
