
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmap/gotosolanelconfirmdialog/GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNo.md")))]
#[::unity2::class(
    namespace = "App.Gmap",
    name = "GoToSolanelConfirmDialog.GoToSolanelConfirmDialogItemNo"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNo {
    #[rename(name = "m_Callback")]
    pub m_callback: crate::system::action::Action,
}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
#[::unity2::methods]
impl GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, callback: crate::system::action::Action, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
impl GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNo {
    pub fn new(callback: crate::system::action::Action, text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IGoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemNoMethods>::ctor(
            this, callback, text,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmap/gotosolanelconfirmdialog/GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App.Gmap",
    name = "GoToSolanelConfirmDialog.GoToSolanelConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYes {
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::action::Action,
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::system::action::Action,
}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
#[::unity2::methods]
impl GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYes {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        decide_callback: crate::system::action::Action,
        cancel_callback: crate::system::action::Action,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
impl GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYes {
    pub fn new(
        decide_callback: crate::system::action::Action,
        cancel_callback: crate::system::action::Action,
        text: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IGoToSolanelConfirmDialog_GoToSolanelConfirmDialogItemYesMethods>::ctor(
            this,
            decide_callback,
            cancel_callback,
            text,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmap/gotosolanelconfirmdialog/GoToSolanelConfirmDialog.md")))]
#[::unity2::class(namespace = "App.Gmap", name = "GoToSolanelConfirmDialog")]
#[parent(crate::system::object::Object)]
pub struct GoToSolanelConfirmDialog {}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
#[::unity2::methods]
impl GoToSolanelConfirmDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        yes_callback: crate::system::action::Action,
        no_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmap-gotosolanelconfirmdialog")]
impl GoToSolanelConfirmDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GoToSolanelConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IGoToSolanelConfirmDialogMethods>::ctor(this);
        this
    }
}
