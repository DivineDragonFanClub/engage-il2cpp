
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstampvisibilitysettingdialog/ProfileCardStampVisibilitySettingDialog.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampVisibilitySettingDialog")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardStampVisibilitySettingDialog {}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
#[::unity2::methods]
impl ProfileCardStampVisibilitySettingDialog {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::basicdialog::BasicDialog;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
impl ProfileCardStampVisibilitySettingDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampVisibilitySettingDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampVisibilitySettingDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstampvisibilitysettingdialog/ProfileCardStampVisibilitySettingDialog_DialogMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardStampVisibilitySettingDialog.DialogMenuItem"
)]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct ProfileCardStampVisibilitySettingDialog_DialogMenuItem {
# [rename (name = "m_DecideEventHandler")] pub m_decide_event_handler : crate :: app :: profilecardstampvisibilitysettingdialog :: ProfileCardStampVisibilitySettingDialog_DecideEventHandler ,
}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
#[::unity2::methods]
impl ProfileCardStampVisibilitySettingDialog_DialogMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        message: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: profilecardstampvisibilitysettingdialog :: ProfileCardStampVisibilitySettingDialog_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
impl ProfileCardStampVisibilitySettingDialog_DialogMenuItem {
    pub fn new(
        message: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: profilecardstampvisibilitysettingdialog :: ProfileCardStampVisibilitySettingDialog_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampVisibilitySettingDialog_DialogMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampVisibilitySettingDialog_DialogMenuItemMethods>::ctor(
            this,
            message,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstampvisibilitysettingdialog/ProfileCardStampVisibilitySettingDialog_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardStampVisibilitySettingDialog.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardStampVisibilitySettingDialog_DecideEventHandler {}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
#[::unity2::methods]
impl ProfileCardStampVisibilitySettingDialog_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardstampvisibilitysettingdialog")]
impl ProfileCardStampVisibilitySettingDialog_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampVisibilitySettingDialog_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampVisibilitySettingDialog_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
