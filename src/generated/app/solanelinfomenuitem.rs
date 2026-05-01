
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenuitem/SolanelInfoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct SolanelInfoMenuItem {
    #[rename(name = "m_HubAreaData")]
    pub m_hub_area_data: crate::app::hubareadata::HubAreaData,
}

#[cfg(feature = "app-solanelinfomenuitem")]
#[::unity2::methods]
impl SolanelInfoMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "GetHubAreaData", args = 0)]
    pub fn get_hub_area_data(self) -> crate::app::hubareadata::HubAreaData;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-solanelinfomenuitem")]
impl SolanelInfoMenuItem {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuItemMethods>::ctor(this, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenuitem/SolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SolanelInfoMenuItem.ConfirmDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct SolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItem {
    #[rename(name = "m_HubAreaData")]
    pub m_hub_area_data: crate::app::hubareadata::HubAreaData,
}

#[cfg(feature = "app-solanelinfomenuitem")]
#[::unity2::methods]
impl SolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        hub_area_data: crate::app::hubareadata::HubAreaData,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-solanelinfomenuitem")]
impl SolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItem {
    pub fn new(
        text: ::unity2::Il2CppString,
        hub_area_data: crate::app::hubareadata::HubAreaData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuItem_ConfirmDialog_ConfirmYesDialogItemMethods>::ctor(
            this,
            text,
            hub_area_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenuitem/SolanelInfoMenuItem_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuItem.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct SolanelInfoMenuItem_ConfirmDialog {}

#[cfg(feature = "app-solanelinfomenuitem")]
#[::unity2::methods]
impl SolanelInfoMenuItem_ConfirmDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        hub_area_data: crate::app::hubareadata::HubAreaData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-solanelinfomenuitem")]
impl SolanelInfoMenuItem_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuItem_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuItem_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}
