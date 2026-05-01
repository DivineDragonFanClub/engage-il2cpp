
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshopbuyemptymenuitem/ItemShopBuyEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopBuyEmptyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ItemShopBuyEmptyMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::itemshopbuymenu::ItemShopBuyMenu_SelectEventHandler,
}

#[cfg(feature = "app-itemshopbuyemptymenuitem")]
#[::unity2::methods]
impl ItemShopBuyEmptyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        select_event_handler: crate::app::itemshopbuymenu::ItemShopBuyMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-itemshopbuyemptymenuitem")]
impl ItemShopBuyEmptyMenuItem {
    pub fn new(
        select_event_handler: crate::app::itemshopbuymenu::ItemShopBuyMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopBuyEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopBuyEmptyMenuItemMethods>::ctor(this, select_event_handler);
        this
    }
}
