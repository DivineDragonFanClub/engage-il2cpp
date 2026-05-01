
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minigameassistdialog/MinigameAssistDialog_AssistItem.md")))]
#[::unity2::class(namespace = "App", name = "MinigameAssistDialog.AssistItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MinigameAssistDialog_AssistItem {}

#[cfg(feature = "app-minigameassistdialog")]
#[::unity2::methods]
impl MinigameAssistDialog_AssistItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-minigameassistdialog")]
impl MinigameAssistDialog_AssistItem {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MinigameAssistDialog_AssistItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMinigameAssistDialog_AssistItemMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minigameassistdialog/MinigameAssistDialog.md")))]
#[::unity2::class(namespace = "App", name = "MinigameAssistDialog")]
#[parent(crate::app::basicdialog::BasicDialog)]
pub struct MinigameAssistDialog {
    #[static_field]
    #[rename(name = "ConfirmLabels")]
    pub confirm_labels: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "YesLabels")]
    pub yes_labels: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "NoLabels")]
    pub no_labels: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "CancelLabels")]
    pub cancel_labels: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-minigameassistdialog")]
#[::unity2::methods]
impl MinigameAssistDialog {
    #[method(name = "get_EventHandler", args = 0)]
    pub fn get_event_handler(
        self,
    ) -> crate::app::minigameassistdialog::MinigameAssistDialog_DecideEventHandler;

    #[method(name = "set_EventHandler", args = 1)]
    pub fn set_event_handler(
        self,
        value: crate::app::minigameassistdialog::MinigameAssistDialog_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
        event_handler: crate::app::minigameassistdialog::MinigameAssistDialog_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        r#type: crate::app::minigameassistdialog::MinigameAssistDialog_MinigameType,
        event_handler: crate::app::minigameassistdialog::MinigameAssistDialog_DecideEventHandler,
    ) -> crate::app::minigameassistdialog::MinigameAssistDialog;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-minigameassistdialog")]
impl MinigameAssistDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
        event_handler: crate::app::minigameassistdialog::MinigameAssistDialog_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MinigameAssistDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMinigameAssistDialogMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minigameassistdialog/MinigameAssistDialog_AssistResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MinigameAssistDialog_AssistResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MinigameAssistDialog_AssistResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MinigameAssistDialog.AssistResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MinigameAssistDialog_AssistResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MinigameAssistDialog_AssistResult {
    pub fn yes() -> Self {
        Self { value: 0 }
    }

    pub fn no() -> Self {
        Self { value: 1 }
    }

    pub fn cancel() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minigameassistdialog/MinigameAssistDialog_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MinigameAssistDialog.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MinigameAssistDialog_DecideEventHandler {}

#[cfg(feature = "app-minigameassistdialog")]
#[::unity2::methods]
impl MinigameAssistDialog_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, set: i32) -> ();
}

#[cfg(feature = "app-minigameassistdialog")]
impl MinigameAssistDialog_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MinigameAssistDialog_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMinigameAssistDialog_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minigameassistdialog/MinigameAssistDialog_MinigameType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MinigameAssistDialog_MinigameType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MinigameAssistDialog_MinigameType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MinigameAssistDialog.MinigameType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MinigameAssistDialog_MinigameType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MinigameAssistDialog_MinigameType {
    pub fn dragon_ride() -> Self {
        Self { value: 0 }
    }

    pub fn fishing() -> Self {
        Self { value: 1 }
    }

    pub fn muscle_exercise() -> Self {
        Self { value: 2 }
    }
}
