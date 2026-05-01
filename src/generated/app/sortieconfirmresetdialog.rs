
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
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmresetdialog/SortieConfirmResetDialog.md")))]
#[::unity2::class(namespace = "App", name = "SortieConfirmResetDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct SortieConfirmResetDialog {}

#[cfg(feature = "app-sortieconfirmresetdialog")]
#[::unity2::methods]
impl SortieConfirmResetDialog {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-sortieconfirmresetdialog")]
impl SortieConfirmResetDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmResetDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmResetDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmresetdialog/SortieConfirmResetDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SortieConfirmResetDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct SortieConfirmResetDialog_ConfirmYesDialogItem {}

#[cfg(feature = "app-sortieconfirmresetdialog")]
#[::unity2::methods]
impl SortieConfirmResetDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortieconfirmresetdialog")]
impl SortieConfirmResetDialog_ConfirmYesDialogItem {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmResetDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmResetDialog_ConfirmYesDialogItemMethods>::ctor(this, text);
        this
    }
}
