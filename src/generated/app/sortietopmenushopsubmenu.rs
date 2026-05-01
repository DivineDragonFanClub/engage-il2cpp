
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenushopsubmenu/SortieTopMenuShopSubMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenuShopSubMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SortieTopMenuShopSubMenu {}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
#[::unity2::methods]
impl SortieTopMenuShopSubMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        parent_menu: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
impl SortieTopMenuShopSubMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenuShopSubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenuShopSubMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenushopsubmenu/SortieTopMenuShopSubMenu_ItemShopMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenuShopSubMenu.ItemShopMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct SortieTopMenuShopSubMenu_ItemShopMenuItem {}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
#[::unity2::methods]
impl SortieTopMenuShopSubMenu_ItemShopMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
impl SortieTopMenuShopSubMenu_ItemShopMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenuShopSubMenu_ItemShopMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenuShopSubMenu_ItemShopMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenushopsubmenu/SortieTopMenuShopSubMenu_WeaponShopMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SortieTopMenuShopSubMenu.WeaponShopMenuItem"
)]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct SortieTopMenuShopSubMenu_WeaponShopMenuItem {}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
#[::unity2::methods]
impl SortieTopMenuShopSubMenu_WeaponShopMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenushopsubmenu")]
impl SortieTopMenuShopSubMenu_WeaponShopMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenuShopSubMenu_WeaponShopMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenuShopSubMenu_WeaponShopMenuItemMethods>::ctor(this);
        this
    }
}
