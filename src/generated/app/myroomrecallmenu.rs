
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_RelianceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.RelianceMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRecallMenu_RelianceMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_RelianceMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_RelianceMenuItem {
    pub fn new(
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_RelianceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_RelianceMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_MovieMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.MovieMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRecallMenu_MovieMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_MovieMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_MovieMenuItem {
    pub fn new(
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_MovieMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_MovieMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_MusicMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.MusicMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRecallMenu_MusicMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_MusicMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_MusicMenuItem {
    pub fn new(
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_MusicMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_MusicMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_GodRelianceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.GodRelianceMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRecallMenu_GodRelianceMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_GodRelianceMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_GodRelianceMenuItem {
    pub fn new(
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_GodRelianceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_GodRelianceMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_WakeupMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.WakeupMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRecallMenu_WakeupMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_WakeupMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_WakeupMenuItem {
    pub fn new(
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_WakeupMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_WakeupMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MyRoomRecallMenu_DecideEventHandler {}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::myroomrecallmenu::MyRoomRecallMenu_MenuResult) -> ();
}

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRecallMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomRecallMenu {}

#[cfg(feature = "app-myroomrecallmenu")]
#[::unity2::methods]
impl MyRoomRecallMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
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
        initial_selected: crate::app::myroomrecallmenu::MyRoomRecallMenu_MenuResult,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::myroomrecallmenu::MyRoomRecallMenu_MenuResult,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> ();
}

#[cfg(feature = "app-myroomrecallmenu")]
impl MyRoomRecallMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::myroomrecallmenu::MyRoomRecallMenu_MenuResult,
        event_handler: crate::app::myroomrecallmenu::MyRoomRecallMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRecallMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRecallMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrecallmenu/MyRoomRecallMenu_MenuResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomRecallMenu_MenuResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomRecallMenu_MenuResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomRecallMenu.MenuResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomRecallMenu_MenuResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomRecallMenu_MenuResult {
    pub fn reliance() -> Self {
        Self { value: 0 }
    }

    pub fn god_reliance() -> Self {
        Self { value: 1 }
    }

    pub fn wakeup() -> Self {
        Self { value: 2 }
    }

    pub fn movie() -> Self {
        Self { value: 3 }
    }

    pub fn music() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}
