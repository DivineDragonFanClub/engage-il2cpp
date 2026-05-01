
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomTopMenu {}

#[cfg(feature = "app-myroomtopmenu")]
#[::unity2::methods]
impl MyRoomTopMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::myroomtopmenu::MyRoomTopMenu_MenuResult,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::myroomtopmenu::MyRoomTopMenu_MenuResult,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();
}

#[cfg(feature = "app-myroomtopmenu")]
impl MyRoomTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::myroomtopmenu::MyRoomTopMenu_MenuResult,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu_MenuResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomTopMenu_MenuResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomTopMenu_MenuResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomTopMenu.MenuResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomTopMenu_MenuResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomTopMenu_MenuResult {
    pub fn sleep() -> Self {
        Self { value: 0 }
    }

    pub fn recall() -> Self {
        Self { value: 1 }
    }

    pub fn set_difficulty() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MyRoomTopMenu_DecideEventHandler {}

#[cfg(feature = "app-myroomtopmenu")]
#[::unity2::methods]
impl MyRoomTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::myroomtopmenu::MyRoomTopMenu_MenuResult) -> ();
}

#[cfg(feature = "app-myroomtopmenu")]
impl MyRoomTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu_RecallSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomTopMenu.RecallSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomTopMenu_RecallSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomtopmenu")]
#[::unity2::methods]
impl MyRoomTopMenu_RecallSelectMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-myroomtopmenu")]
impl MyRoomTopMenu_RecallSelectMenuItem {
    pub fn new(event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomTopMenu_RecallSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomTopMenu_RecallSelectMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu_SleepSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomTopMenu.SleepSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomTopMenu_SleepSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomtopmenu")]
#[::unity2::methods]
impl MyRoomTopMenu_SleepSelectMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-myroomtopmenu")]
impl MyRoomTopMenu_SleepSelectMenuItem {
    pub fn new(event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomTopMenu_SleepSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomTopMenu_SleepSelectMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomtopmenu/MyRoomTopMenu_SetDifficultySelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomTopMenu.SetDifficultySelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomTopMenu_SetDifficultySelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomtopmenu")]
#[::unity2::methods]
impl MyRoomTopMenu_SetDifficultySelectMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-myroomtopmenu")]
impl MyRoomTopMenu_SetDifficultySelectMenuItem {
    pub fn new(event_handler: crate::app::myroomtopmenu::MyRoomTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomTopMenu_SetDifficultySelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomTopMenu_SetDifficultySelectMenuItemMethods>::ctor(this, event_handler);
        this
    }
}
