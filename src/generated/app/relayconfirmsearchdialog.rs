
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayconfirmsearchdialog/RelayConfirmSearchDialog_YesMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RelayConfirmSearchDialog.YesMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RelayConfirmSearchDialog_YesMenuItem {}

#[cfg(feature = "app-relayconfirmsearchdialog")]
#[::unity2::methods]
impl RelayConfirmSearchDialog_YesMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-relayconfirmsearchdialog")]
impl RelayConfirmSearchDialog_YesMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayConfirmSearchDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayConfirmSearchDialog_YesMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayconfirmsearchdialog/RelayConfirmSearchDialog_NoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RelayConfirmSearchDialog.NoMenuItem")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct RelayConfirmSearchDialog_NoMenuItem {}

#[cfg(feature = "app-relayconfirmsearchdialog")]
#[::unity2::methods]
impl RelayConfirmSearchDialog_NoMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-relayconfirmsearchdialog")]
impl RelayConfirmSearchDialog_NoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayConfirmSearchDialog_NoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayConfirmSearchDialog_NoMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayconfirmsearchdialog/RelayConfirmSearchDialog.md")))]
#[::unity2::class(namespace = "App", name = "RelayConfirmSearchDialog")]
#[parent(crate::system::object::Object)]
pub struct RelayConfirmSearchDialog {}

#[cfg(feature = "app-relayconfirmsearchdialog")]
#[::unity2::methods]
impl RelayConfirmSearchDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, code: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayconfirmsearchdialog")]
impl RelayConfirmSearchDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayConfirmSearchDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayConfirmSearchDialogMethods>::ctor(this);
        this
    }
}
