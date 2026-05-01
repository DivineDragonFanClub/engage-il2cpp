
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventoryunititemmenuitem/InventoryUnitItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "InventoryUnitItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct InventoryUnitItemMenuItem {
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
}

#[cfg(feature = "app-inventoryunititemmenuitem")]
#[::unity2::methods]
impl InventoryUnitItemMenuItem {
    #[method(name = "get_m_SelectableBlank", args = 0)]
    pub fn get_m_selectable_blank(self) -> bool;

    #[method(name = "set_m_SelectableBlank", args = 1)]
    pub fn set_m_selectable_blank(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, owner_item_index: i32, selectable_blank: bool) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "IsEffective", args = 0)]
    pub fn is_effective(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetRecieverUnit", args = 0)]
    pub fn get_reciever_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItemIndex", args = 0)]
    pub fn get_unit_item_index(self) -> i32;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "AddSelection", args = 0)]
    pub fn add_selection(self) -> ();

    #[method(name = "RemoveSelection", args = 0)]
    pub fn remove_selection(self) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "GoToSubMenu", args = 0)]
    pub fn go_to_sub_menu(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GoToTrade", args = 0)]
    pub fn go_to_trade(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GoToPoolMenu", args = 0)]
    pub fn go_to_pool_menu(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "DirectTrade", args = 0)]
    pub fn direct_trade(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-inventoryunititemmenuitem")]
impl InventoryUnitItemMenuItem {
    pub fn new(owner_item_index: i32, selectable_blank: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryUnitItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryUnitItemMenuItemMethods>::ctor(this, owner_item_index, selectable_blank);
        this
    }
}
