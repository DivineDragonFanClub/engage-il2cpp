
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
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymapmenu/RelayMapMenu_MapMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RelayMapMenu.MapMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RelayMapMenu_MapMenuItem {
    #[rename(name = "m_Rdata")]
    pub m_rdata: crate::app::relaydata::RelayData,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::relaymapmenu::RelayMapMenu_SelectEventHandler,
}

#[cfg(feature = "app-relaymapmenu")]
#[::unity2::methods]
impl RelayMapMenu_MapMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        cdata: crate::app::chapterdata::ChapterData,
        select_event_handler: crate::app::relaymapmenu::RelayMapMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-relaymapmenu")]
impl RelayMapMenu_MapMenuItem {
    pub fn new(
        cdata: crate::app::chapterdata::ChapterData,
        select_event_handler: crate::app::relaymapmenu::RelayMapMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMapMenu_MapMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMapMenu_MapMenuItemMethods>::ctor(this, cdata, select_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymapmenu/RelayMapMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RelayMapMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RelayMapMenu_DecideEventHandler {}

#[cfg(feature = "app-relaymapmenu")]
#[::unity2::methods]
impl RelayMapMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-relaymapmenu")]
impl RelayMapMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMapMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMapMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymapmenu/RelayMapMenu.md")))]
#[::unity2::class(namespace = "App", name = "RelayMapMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RelayMapMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::relaymapmenu::RelayMapMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::relaymapmenu::RelayMapMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-relaymapmenu")]
#[::unity2::methods]
impl RelayMapMenu {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        root_object: crate::unity_engine::gameobject::GameObject,
        select_event_handler: crate::app::relaymapmenu::RelayMapMenu_SelectEventHandler,
        decide_event_handler: crate::app::relaymapmenu::RelayMapMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: relaymapmenu :: RelayMapMenu_RequestCloseEventHandler,
    ) -> crate::app::relaymapmenu::RelayMapMenu;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::relaymapmenucontent::RelayMapMenuContent,
        decide_event_handler: crate::app::relaymapmenu::RelayMapMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: relaymapmenu :: RelayMapMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-relaymapmenu")]
impl RelayMapMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::relaymapmenucontent::RelayMapMenuContent,
        decide_event_handler: crate::app::relaymapmenu::RelayMapMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: relaymapmenu :: RelayMapMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMapMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMapMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymapmenu/RelayMapMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RelayMapMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RelayMapMenu_SelectEventHandler {}

#[cfg(feature = "app-relaymapmenu")]
#[::unity2::methods]
impl RelayMapMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, relay_data: crate::app::relaydata::RelayData) -> ();
}

#[cfg(feature = "app-relaymapmenu")]
impl RelayMapMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMapMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMapMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymapmenu/RelayMapMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RelayMapMenu.RequestCloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RelayMapMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-relaymapmenu")]
#[::unity2::methods]
impl RelayMapMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-relaymapmenu")]
impl RelayMapMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMapMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMapMenu_RequestCloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
