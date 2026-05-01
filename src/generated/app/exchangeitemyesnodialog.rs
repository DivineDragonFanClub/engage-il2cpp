
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeitemyesnodialog/ExchangeItemYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeItemYesNoDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct ExchangeItemYesNoDialog {}

#[cfg(feature = "app-exchangeitemyesnodialog")]
#[::unity2::methods]
impl ExchangeItemYesNoDialog {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unititem::UnitItem,
        >,
        expected: i32,
        yes_event_handler : crate :: app :: exchangeitemyesnodialog :: ExchangeItemYesNoDialog_YesEventHandler,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();
}

#[cfg(feature = "app-exchangeitemyesnodialog")]
impl ExchangeItemYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeItemYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeItemYesNoDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeitemyesnodialog/ExchangeItemYesNoDialog_YesEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeItemYesNoDialog.YesEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ExchangeItemYesNoDialog_YesEventHandler {}

#[cfg(feature = "app-exchangeitemyesnodialog")]
#[::unity2::methods]
impl ExchangeItemYesNoDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-exchangeitemyesnodialog")]
impl ExchangeItemYesNoDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeItemYesNoDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeItemYesNoDialog_YesEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeitemyesnodialog/ExchangeItemYesNoDialog_YesMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeItemYesNoDialog.YesMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ExchangeItemYesNoDialog_YesMenuItem {
    #[rename(name = "m_YesEventHandler")]
    pub m_yes_event_handler:
        crate::app::exchangeitemyesnodialog::ExchangeItemYesNoDialog_YesEventHandler,
}

#[cfg(feature = "app-exchangeitemyesnodialog")]
#[::unity2::methods]
impl ExchangeItemYesNoDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: exchangeitemyesnodialog :: ExchangeItemYesNoDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-exchangeitemyesnodialog")]
impl ExchangeItemYesNoDialog_YesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: exchangeitemyesnodialog :: ExchangeItemYesNoDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeItemYesNoDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeItemYesNoDialog_YesMenuItemMethods>::ctor(this, yes_event_handler);
        this
    }
}
