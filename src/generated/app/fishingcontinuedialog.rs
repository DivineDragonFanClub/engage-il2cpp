
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingcontinuedialog/FishingContinueDialog_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "FishingContinueDialog.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct FishingContinueDialog_DecideEventHandler {}

#[cfg(feature = "app-fishingcontinuedialog")]
#[::unity2::methods]
impl FishingContinueDialog_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: bool) -> ();
}

#[cfg(feature = "app-fishingcontinuedialog")]
impl FishingContinueDialog_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingContinueDialog_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingContinueDialog_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingcontinuedialog/FishingContinueDialog.md")))]
#[::unity2::class(namespace = "App", name = "FishingContinueDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct FishingContinueDialog {}

#[cfg(feature = "app-fishingcontinuedialog")]
#[::unity2::methods]
impl FishingContinueDialog {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::fishingcontinuedialog::FishingContinueDialog_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::fishingcontinuedialog::FishingContinueDialog_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
        event_handler: crate::app::fishingcontinuedialog::FishingContinueDialog_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        event_handler: crate::app::fishingcontinuedialog::FishingContinueDialog_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-fishingcontinuedialog")]
impl FishingContinueDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
        event_handler: crate::app::fishingcontinuedialog::FishingContinueDialog_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingContinueDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingContinueDialogMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            event_handler,
        );
        this
    }
}
