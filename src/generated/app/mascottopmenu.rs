
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MascotTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MascotTopMenu_DecideEventHandler {}

#[cfg(feature = "app-mascottopmenu")]
#[::unity2::methods]
impl MascotTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::mascottopmenu::MascotTopMenu_MenuResult) -> ();
}

#[cfg(feature = "app-mascottopmenu")]
impl MascotTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu_CustomMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MascotTopMenu.CustomMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MascotTopMenu_CustomMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-mascottopmenu")]
#[::unity2::methods]
impl MascotTopMenu_CustomMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
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

#[cfg(feature = "app-mascottopmenu")]
impl MascotTopMenu_CustomMenuItem {
    pub fn new(event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotTopMenu_CustomMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotTopMenu_CustomMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu_StrokMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MascotTopMenu.StrokMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MascotTopMenu_StrokMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-mascottopmenu")]
#[::unity2::methods]
impl MascotTopMenu_StrokMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
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

#[cfg(feature = "app-mascottopmenu")]
impl MascotTopMenu_StrokMenuItem {
    pub fn new(event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotTopMenu_StrokMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotTopMenu_StrokMenuItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "MascotTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MascotTopMenu {}

#[cfg(feature = "app-mascottopmenu")]
#[::unity2::methods]
impl MascotTopMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
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
        initial_selected: crate::app::mascottopmenu::MascotTopMenu_MenuResult,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::mascottopmenu::MascotTopMenu_MenuResult,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
    ) -> ();
}

#[cfg(feature = "app-mascottopmenu")]
impl MascotTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::mascottopmenu::MascotTopMenu_MenuResult,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu_MenuResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MascotTopMenu_MenuResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MascotTopMenu_MenuResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MascotTopMenu.MenuResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MascotTopMenu_MenuResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MascotTopMenu_MenuResult {
    pub fn meal() -> Self {
        Self { value: 0 }
    }

    pub fn strok() -> Self {
        Self { value: 1 }
    }

    pub fn custom() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascottopmenu/MascotTopMenu_MealMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MascotTopMenu.MealMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MascotTopMenu_MealMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-mascottopmenu")]
#[::unity2::methods]
impl MascotTopMenu_MealMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler,
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

#[cfg(feature = "app-mascottopmenu")]
impl MascotTopMenu_MealMenuItem {
    pub fn new(event_handler: crate::app::mascottopmenu::MascotTopMenu_DecideEventHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotTopMenu_MealMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotTopMenu_MealMenuItemMethods>::ctor(this, event_handler);
        this
    }
}
