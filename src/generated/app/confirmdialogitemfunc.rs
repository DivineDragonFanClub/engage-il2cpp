
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/confirmdialogitemfunc/ConfirmDialogItemFunc.md")))]
#[::unity2::class(namespace = "App", name = "ConfirmDialogItemFunc")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct ConfirmDialogItemFunc {
    #[rename(name = "m_func")]
    pub m_func: crate::system::action::Action,
}

#[cfg(feature = "app-confirmdialogitemfunc")]
#[::unity2::methods]
impl ConfirmDialogItemFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, label: ::unity2::Il2CppString, func: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-confirmdialogitemfunc")]
impl ConfirmDialogItemFunc {
    pub fn new(label: ::unity2::Il2CppString, func: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConfirmDialogItemFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IConfirmDialogItemFuncMethods>::ctor(this, label, func);
        this
    }
}
