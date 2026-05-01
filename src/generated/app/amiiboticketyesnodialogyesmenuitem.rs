
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiiboticketyesnodialogyesmenuitem/AmiiboTicketYesNoDialogYesMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboTicketYesNoDialogYesMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct AmiiboTicketYesNoDialogYesMenuItem {
    #[rename(name = "m_YesEventHandler")]
    pub m_yes_event_handler: crate::system::action::Action,
}

#[cfg(feature = "app-amiiboticketyesnodialogyesmenuitem")]
#[::unity2::methods]
impl AmiiboTicketYesNoDialogYesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, yes_event_handler: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-amiiboticketyesnodialogyesmenuitem")]
impl AmiiboTicketYesNoDialogYesMenuItem {
    pub fn new(yes_event_handler: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboTicketYesNoDialogYesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboTicketYesNoDialogYesMenuItemMethods>::ctor(this, yes_event_handler);
        this
    }
}
