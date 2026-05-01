
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trademenuitem/TradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "TradeMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct TradeMenuItem {}

#[cfg(feature = "app-trademenuitem")]
#[::unity2::methods]
impl TradeMenuItem {
    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetTradeMenu", args = 0)]
    pub fn get_trade_menu(self) -> crate::app::trademenu::TradeMenu;

    #[method(name = "GetLeftUnit", args = 0)]
    pub fn get_left_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetRightUnit", args = 0)]
    pub fn get_right_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetLeftInitialSelectIndex", args = 0)]
    pub fn get_left_initial_select_index(self) -> i32;

    #[method(name = "GetRightInitialSelectIndex", args = 0)]
    pub fn get_right_initial_select_index(self) -> i32;

    #[method(name = "SetDone", args = 0)]
    pub fn set_done(self) -> ();

    #[method(name = "IsDone", args = 0)]
    pub fn is_done(self) -> bool;

    #[method(name = "OnTrade", args = 0)]
    pub fn on_trade(self) -> ();

    #[method(name = "OnEnd", args = 0)]
    pub fn on_end(self) -> ();

    #[method(name = "BuildAttributeSelect", args = 0)]
    pub fn build_attribute_select(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "Trade", args = 0)]
    pub fn trade(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SelectFirstItem", args = 0)]
    pub fn select_first_item(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-trademenuitem")]
impl TradeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ITradeMenuItemMethods>::ctor(this);
        this
    }
}
