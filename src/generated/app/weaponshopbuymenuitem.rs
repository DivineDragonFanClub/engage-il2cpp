
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshopbuymenuitem/WeaponShopBuyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "WeaponShopBuyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct WeaponShopBuyMenuItem {
    #[rename(name = "m_Iid")]
    pub m_iid: ::unity2::Il2CppString,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_activeTextColor2")]
    pub m_active_text_color2: crate::unity_engine::color::Color,
    #[rename(name = "m_inactiveTextColor2")]
    pub m_inactive_text_color2: crate::unity_engine::color::Color,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_DecideEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::weaponshopbuymenu::WeaponShopBuyMenu_ChangeUnitToPrevEventHandler,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::weaponshopbuymenu::WeaponShopBuyMenu_ChangeUnitToNextEventHandler,
}

#[cfg(feature = "app-weaponshopbuymenuitem")]
#[::unity2::methods]
impl WeaponShopBuyMenuItem {
    #[method(name = "get_m_ItemData", args = 0)]
    pub fn get_m_item_data(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_m_ItemData", args = 1)]
    pub fn set_m_item_data(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_m_UnitItem", args = 0)]
    pub fn get_m_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_UnitItem", args = 1)]
    pub fn set_m_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_StockNum", args = 0)]
    pub fn get_m_stock_num(self) -> i32;

    #[method(name = "set_m_StockNum", args = 1)]
    pub fn set_m_stock_num(self, value: i32) -> ();

    #[method(name = "get_m_NewArrival", args = 0)]
    pub fn get_m_new_arrival(self) -> bool;

    #[method(name = "set_m_NewArrival", args = 1)]
    pub fn set_m_new_arrival(self, value: bool) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        shop_content: crate::app::shopcontent::ShopContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_SelectEventHandler,
        decide_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_DecideEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: weaponshopbuymenu :: WeaponShopBuyMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: weaponshopbuymenu :: WeaponShopBuyMenu_ChangeUnitToNextEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "IsEffective", args = 0)]
    pub fn is_effective(self) -> bool;

    #[method(name = "SetTextColor", args = 4)]
    pub fn set_text_color(
        self,
        active_color1: crate::unity_engine::color::Color,
        active_color2: crate::unity_engine::color::Color,
        inactive_color1: crate::unity_engine::color::Color,
        inactive_color2: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDoneToBuy", args = 0)]
    pub fn on_done_to_buy(self) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnitItemEmptyCount", args = 0)]
    pub fn get_unit_item_empty_count(self) -> i32;

    #[method(name = "IsEnoughMoneyToBeBuyed", args = 0)]
    pub fn is_enough_money_to_be_buyed(self) -> bool;
}

#[cfg(feature = "app-weaponshopbuymenuitem")]
impl WeaponShopBuyMenuItem {
    pub fn new(
        shop_content: crate::app::shopcontent::ShopContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_SelectEventHandler,
        decide_event_handler: crate::app::weaponshopbuymenu::WeaponShopBuyMenu_DecideEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: weaponshopbuymenu :: WeaponShopBuyMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: weaponshopbuymenu :: WeaponShopBuyMenu_ChangeUnitToNextEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponShopBuyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponShopBuyMenuItemMethods>::ctor(
            this,
            shop_content,
            unit,
            select_event_handler,
            decide_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
        );
        this
    }
}
