
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenuitem/ShopSellMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ShopSellMenuItem {
    #[rename(name = "m_activeTextColor2")]
    pub m_active_text_color2: crate::unity_engine::color::Color,
    #[rename(name = "m_inactiveTextColor2")]
    pub m_inactive_text_color2: crate::unity_engine::color::Color,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
    #[rename(name = "m_DecideItemEventHandler")]
    pub m_decide_item_event_handler:
        crate::app::shopsellmenuitem::ShopSellMenuItem_DecideItemEventHandler,
    #[rename(name = "m_CancelItemEventHandler")]
    pub m_cancel_item_event_handler:
        crate::app::shopsellmenuitem::ShopSellMenuItem_CancelItemEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToPrevEventHandler,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToNextEventHandler,
}

#[cfg(feature = "app-shopsellmenuitem")]
#[::unity2::methods]
impl ShopSellMenuItem {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_m_ItemIndex", args = 0)]
    pub fn get_m_item_index(self) -> i32;

    #[method(name = "set_m_ItemIndex", args = 1)]
    pub fn set_m_item_index(self, value: i32) -> ();

    #[method(name = "get_m_UnitItem", args = 0)]
    pub fn get_m_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_UnitItem", args = 1)]
    pub fn set_m_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_Decided", args = 0)]
    pub fn get_m_decided(self) -> bool;

    #[method(name = "set_m_Decided", args = 1)]
    pub fn set_m_decided(self, value: bool) -> ();

    #[method(name = "get_m_SortValue", args = 0)]
    pub fn get_m_sort_value(self) -> i64;

    #[method(name = "set_m_SortValue", args = 1)]
    pub fn set_m_sort_value(self, value: i64) -> ();

    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        item_index: i32,
        decided: bool,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_DecideItemEventHandler,
        cancel_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
    ) -> ();

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
        active_color: crate::unity_engine::color::Color,
        active_color2: crate::unity_engine::color::Color,
        inactive_color: crate::unity_engine::color::Color,
        inactive_color2: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetDecide", args = 0)]
    pub fn set_decide(self) -> ();

    #[method(name = "SetNotDecide", args = 0)]
    pub fn set_not_decide(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "SetDecideItemEventHandler", args = 1)]
    pub fn set_decide_item_event_handler(
        self,
        decide_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_DecideItemEventHandler,
    ) -> ();

    #[method(name = "SetCancelItemEventHandler", args = 1)]
    pub fn set_cancel_item_event_handler(
        self,
        cancel_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_CancelItemEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-shopsellmenuitem")]
impl ShopSellMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        item_index: i32,
        decided: bool,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        decide_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_DecideItemEventHandler,
        cancel_item_event_handler : crate :: app :: shopsellmenuitem :: ShopSellMenuItem_CancelItemEventHandler,
        decide_event_handler: crate::app::shopsellmenu::ShopSellMenu_DecideEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuItemMethods>::ctor(
            this,
            unit,
            item_index,
            decided,
            select_event_handler,
            decide_item_event_handler,
            cancel_item_event_handler,
            decide_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenuitem/ShopSellMenuItem_CancelItemEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuItem.CancelItemEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenuItem_CancelItemEventHandler {}

#[cfg(feature = "app-shopsellmenuitem")]
#[::unity2::methods]
impl ShopSellMenuItem_CancelItemEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, item_index: i32) -> ();
}

#[cfg(feature = "app-shopsellmenuitem")]
impl ShopSellMenuItem_CancelItemEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuItem_CancelItemEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuItem_CancelItemEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenuitem/ShopSellMenuItem_DecideItemEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuItem.DecideItemEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ShopSellMenuItem_DecideItemEventHandler {}

#[cfg(feature = "app-shopsellmenuitem")]
#[::unity2::methods]
impl ShopSellMenuItem_DecideItemEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, item_index: i32) -> ();
}

#[cfg(feature = "app-shopsellmenuitem")]
impl ShopSellMenuItem_DecideItemEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuItem_DecideItemEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuItem_DecideItemEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
