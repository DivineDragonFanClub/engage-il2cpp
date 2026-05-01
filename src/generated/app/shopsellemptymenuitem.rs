
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellemptymenuitem/ShopSellEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellEmptyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ShopSellEmptyMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
    #[rename(name = "m_ChangeUnitToPrevEventHandler")]
    pub m_change_unit_to_prev_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToPrevEventHandler,
    #[rename(name = "m_ChangeUnitToNextEventHandler")]
    pub m_change_unit_to_next_event_handler:
        crate::app::shopsellmenu::ShopSellMenu_ChangeUnitToNextEventHandler,
}

#[cfg(feature = "app-shopsellemptymenuitem")]
#[::unity2::methods]
impl ShopSellEmptyMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-shopsellemptymenuitem")]
impl ShopSellEmptyMenuItem {
    pub fn new(
        select_event_handler: crate::app::shopsellmenu::ShopSellMenu_SelectEventHandler,
        change_unit_to_prev_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: shopsellmenu :: ShopSellMenu_ChangeUnitToNextEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellEmptyMenuItemMethods>::ctor(
            this,
            select_event_handler,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
        );
        this
    }
}
