
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_StoreAllMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectAllUnitMenu.StoreAllMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct UnitSelectAllUnitMenu_StoreAllMenuItem {
    #[rename(name = "m_IsSuccess")]
    pub m_is_success: bool,
}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_StoreAllMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "IsTarget", args = 1)]
    pub fn is_target(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_StoreAllMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_StoreAllMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_StoreAllMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct UnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItem {
    #[rename(name = "m_DecideEventHander")]
    pub m_decide_event_hander:
        crate::app::unitselectallunitmenu::UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler,
}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        decide : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItem {
    pub fn new(
        text: ::unity2::Il2CppString,
        decide : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmDialog_ConfirmYesDialogItemMethods>::ctor(
            this, text, decide,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmSequence.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectAllUnitMenu.ConfirmSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmSequence {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmSequence_DecideEventHandler,
        check : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler,
        failure : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler,
        end : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmDialog.CloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmDialog_CloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmSequence.CheckEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> bool;
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmSequence_CheckEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectAllUnitMenu.ConfirmDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct UnitSelectAllUnitMenu_ConfirmDialog {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        msg: ::unity2::Il2CppString,
        yes: ::unity2::Il2CppString,
        decide : crate :: app :: unitselectallunitmenu :: UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmDialogMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmDialog.ConfirmNoDialogItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct UnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItem {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItem {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmDialog_ConfirmNoDialogItemMethods>::ctor(this, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectAllUnitMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct UnitSelectAllUnitMenu {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu {
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

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenuMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_EntrustMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectAllUnitMenu.EntrustMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct UnitSelectAllUnitMenu_EntrustMenuItem {
    #[rename(name = "m_Result")]
    pub m_result: crate::app::sortieentrustresult::SortieEntrustResult,
}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_EntrustMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_EntrustMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_EntrustMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_EntrustMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitSelectAllUnitMenu_ConfirmSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitSelectAllUnitMenu.ConfirmSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitSelectAllUnitMenu_ConfirmSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitSelectAllUnitMenu_ConfirmSequence_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmDialog.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmDialog_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmSequence.EndEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmSequence_EndEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmSequence_EndEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectallunitmenu/UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitSelectAllUnitMenu.ConfirmSequence.FailureEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler {}

#[cfg(feature = "app-unitselectallunitmenu")]
#[::unity2::methods]
impl UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-unitselectallunitmenu")]
impl UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectAllUnitMenu_ConfirmSequence_FailureEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
