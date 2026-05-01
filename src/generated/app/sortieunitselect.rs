
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieunitselect/SortieUnitSelect_UnitEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieUnitSelect.UnitEmptyMenuItem")]
#[parent(crate::app::sortieunitselect::SortieUnitSelect_UnitMenuItem)]
pub struct SortieUnitSelect_UnitEmptyMenuItem {}

#[cfg(feature = "app-sortieunitselect")]
#[::unity2::methods]
impl SortieUnitSelect_UnitEmptyMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-sortieunitselect")]
impl SortieUnitSelect_UnitEmptyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUnitSelect_UnitEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieUnitSelect_UnitEmptyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieunitselect/SortieUnitSelect.md")))]
#[::unity2::class(namespace = "App", name = "SortieUnitSelect")]
#[parent(crate::system::object::Object)]
pub struct SortieUnitSelect {}

#[cfg(feature = "app-sortieunitselect")]
#[::unity2::methods]
impl SortieUnitSelect {
    #[method(name = "GetSelectIndexFromUnit", args = 2)]
    pub fn get_select_index_from_unit(
        menu: crate::app::basicmenu::BasicMenu,
        unit: crate::app::unit::Unit,
    ) -> i32;

    #[method(name = "CreateMenuItemListCommon", args = 0)]
    pub fn create_menu_item_list_common(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "CreateMenuItemList", args = 0)]
    pub fn create_menu_item_list(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "CreateMenuItemListForRelay", args = 0)]
    pub fn create_menu_item_list_for_relay(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "CreateMenuItemListForVersus", args = 0)]
    pub fn create_menu_item_list_for_versus(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "CanTrade", args = 1)]
    pub fn can_trade(to_unit: crate::app::unit::Unit) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieunitselect")]
impl SortieUnitSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUnitSelect),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieUnitSelectMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieunitselect/SortieUnitSelect_ConfirmBattleSequence.md")))]
#[::unity2::class(namespace = "App", name = "SortieUnitSelect.ConfirmBattleSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct SortieUnitSelect_ConfirmBattleSequence {
    #[rename(name = "m_ParentMenu")]
    pub m_parent_menu: crate::app::basicmenu::BasicMenu,
}

#[cfg(feature = "app-sortieunitselect")]
#[::unity2::methods]
impl SortieUnitSelect_ConfirmBattleSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(parent_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, parent_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateDialog", args = 0)]
    pub fn create_dialog(self) -> ();
}

#[cfg(feature = "app-sortieunitselect")]
impl SortieUnitSelect_ConfirmBattleSequence {
    pub fn new(parent_menu: crate::app::basicmenu::BasicMenu) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUnitSelect_ConfirmBattleSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieUnitSelect_ConfirmBattleSequenceMethods>::ctor(this, parent_menu);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieunitselect/SortieUnitSelect_UnitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SortieUnitSelect.UnitMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct SortieUnitSelect_UnitMenuItem {
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_CanSortieForRelay")]
    pub m_can_sortie_for_relay: bool,
}

#[cfg(feature = "app-sortieunitselect")]
#[::unity2::methods]
impl SortieUnitSelect_UnitMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, unit: crate::app::unit::Unit, can_sortie_for_relay: bool) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "CreateSubMenu", args = 2)]
    pub fn create_sub_menu(
        by_decision: bool,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

    #[method(name = "CancelInventoryTrade", args = 0)]
    pub fn cancel_inventory_trade(self) -> ();
}

#[cfg(feature = "app-sortieunitselect")]
impl SortieUnitSelect_UnitMenuItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUnitSelect_UnitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieUnitSelect_UnitMenuItemMethods>::ctor(this, unit);
        this
    }

    pub fn new_2(unit: crate::app::unit::Unit, can_sortie_for_relay: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUnitSelect_UnitMenuItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISortieUnitSelect_UnitMenuItemMethods>::ctor_2(this, unit, can_sortie_for_relay);
        this
    }
}
