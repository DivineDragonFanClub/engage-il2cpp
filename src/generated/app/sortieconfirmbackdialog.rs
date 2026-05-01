
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmbackdialog/SortieConfirmBackDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SortieConfirmBackDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct SortieConfirmBackDialog_ConfirmYesDialogItem {}

#[cfg(feature = "app-sortieconfirmbackdialog")]
#[::unity2::methods]
impl SortieConfirmBackDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortieconfirmbackdialog")]
impl SortieConfirmBackDialog_ConfirmYesDialogItem {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmBackDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmBackDialog_ConfirmYesDialogItemMethods>::ctor(this, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmbackdialog/SortieConfirmBackDialog.md")))]
#[::unity2::class(namespace = "App", name = "SortieConfirmBackDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct SortieConfirmBackDialog {}

#[cfg(feature = "app-sortieconfirmbackdialog")]
#[::unity2::methods]
impl SortieConfirmBackDialog {
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

#[cfg(feature = "app-sortieconfirmbackdialog")]
impl SortieConfirmBackDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmBackDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmBackDialogMethods>::ctor(this, menu_item_list);
        this
    }
}
