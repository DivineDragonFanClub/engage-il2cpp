
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SendItemMenu {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_GainUnitItem")]
    pub m_gain_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::senditemroot::SendItemRoot,
    #[rename(name = "m_Uncancellable")]
    pub m_uncancellable: bool,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::app::senditemmenu::SendItemMenu_CancelCallback,
}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        gain_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
        cancel_callback: crate::app::senditemmenu::SendItemMenu_CancelCallback,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::senditemroot::SendItemRoot,
        unit: crate::app::unit::Unit,
        gain_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
        cancel_callback: crate::app::senditemmenu::SendItemMenu_CancelCallback,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetGainUnitItem", args = 0)]
    pub fn get_gain_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "SendItem", args = 2)]
    pub fn send_item(self, select_unit_item: bool, unit_item_index: i32) -> ();
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::senditemroot::SendItemRoot,
        unit: crate::app::unit::Unit,
        gain_unit_item: crate::app::unititem::UnitItem,
        uncancellable: bool,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
        cancel_callback: crate::app::senditemmenu::SendItemMenu_CancelCallback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            root,
            unit,
            gain_unit_item,
            uncancellable,
            decide_callback,
            cancel_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu_DecideCallback.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenu.DecideCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SendItemMenu_DecideCallback {}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu_DecideCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, select_unit_item: bool, unit_item_index: i32) -> ();
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu_DecideCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu_DecideCallback),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenu_DecideCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu_ConfirmDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SendItemMenu.ConfirmDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct SendItemMenu_ConfirmDialog_ConfirmYesDialogItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
    #[rename(name = "m_GainUnitItem")]
    pub m_gain_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu_ConfirmDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        gain_unit_item: crate::app::unititem::UnitItem,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu_ConfirmDialog_ConfirmYesDialogItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        gain_unit_item: crate::app::unititem::UnitItem,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu_ConfirmDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenu_ConfirmDialog_ConfirmYesDialogItemMethods>::ctor(
            this,
            unit,
            unit_item_index,
            gain_unit_item,
            decide_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct SendItemMenu_ConfirmDialog {}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        gain_unit_item: crate::app::unititem::UnitItem,
        decide_callback: crate::app::senditemmenu::SendItemMenu_DecideCallback,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenu_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu_CancelCallback.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenu.CancelCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SendItemMenu_CancelCallback {}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu_CancelCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu_CancelCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu_CancelCallback),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenu_CancelCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenu/SendItemMenu_SendItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenu.SendItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct SendItemMenu_SendItemMenuItem {}

#[cfg(feature = "app-senditemmenu")]
#[::unity2::methods]
impl SendItemMenu_SendItemMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetSendItemMenu", args = 0)]
    pub fn get_send_item_menu(self) -> crate::app::senditemmenu::SendItemMenu;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetRecieverUnit", args = 0)]
    pub fn get_reciever_unit(self) -> crate::app::unit::Unit;

    #[method(name = "IsGainItem", args = 0)]
    pub fn is_gain_item(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-senditemmenu")]
impl SendItemMenu_SendItemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenu_SendItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenu_SendItemMenuItemMethods>::ctor(this);
        this
    }
}
