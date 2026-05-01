
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshopbuyyesnodialogyesmenuitem/ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ItemShopBuyYesNoDialogYesMenuItem.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler {}

#[cfg(feature = "app-itemshopbuyyesnodialogyesmenuitem")]
#[::unity2::methods]
impl ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-itemshopbuyyesnodialogyesmenuitem")]
impl ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopBuyYesNoDialogYesMenuItem_YesEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshopbuyyesnodialogyesmenuitem/ItemShopBuyYesNoDialogYesMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopBuyYesNoDialogYesMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ItemShopBuyYesNoDialogYesMenuItem {
# [rename (name = "m_YesEventHandler")] pub m_yes_event_handler : crate :: app :: itemshopbuyyesnodialogyesmenuitem :: ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler ,
}

#[cfg(feature = "app-itemshopbuyyesnodialogyesmenuitem")]
#[::unity2::methods]
impl ItemShopBuyYesNoDialogYesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: itemshopbuyyesnodialogyesmenuitem :: ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-itemshopbuyyesnodialogyesmenuitem")]
impl ItemShopBuyYesNoDialogYesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: itemshopbuyyesnodialogyesmenuitem :: ItemShopBuyYesNoDialogYesMenuItem_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopBuyYesNoDialogYesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopBuyYesNoDialogYesMenuItemMethods>::ctor(this, yes_event_handler);
        this
    }
}
