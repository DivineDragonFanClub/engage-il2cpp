
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.BaseMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct UnitSelectSubMenu_BaseMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_BaseMenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_BaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_BaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_SkillMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.SkillMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_SkillMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_SkillMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_SkillMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_SkillMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_SkillMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_TradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.TradeMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_TradeMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_TradeMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_TradeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_TradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_TradeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct UnitSelectSubMenu {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
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

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenuMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_EntrustMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.EntrustMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_EntrustMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_EntrustMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_EntrustMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_EntrustMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_EntrustMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_ClassChangeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.ClassChangeMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_ClassChangeMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_ClassChangeMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_ClassChangeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_ClassChangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_ClassChangeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_InventoryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.InventoryMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_InventoryMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_InventoryMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_InventoryMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_InventoryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_InventoryMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsubmenu/UnitSelectSubMenu_StoreAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSubMenu.StoreAllMenuItem")]
#[parent(crate::app::unitselectsubmenu::UnitSelectSubMenu_BaseMenuItem)]
pub struct UnitSelectSubMenu_StoreAllMenuItem {}

#[cfg(feature = "app-unitselectsubmenu")]
#[::unity2::methods]
impl UnitSelectSubMenu_StoreAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsubmenu")]
impl UnitSelectSubMenu_StoreAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSubMenu_StoreAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSubMenu_StoreAllMenuItemMethods>::ctor(this);
        this
    }
}
