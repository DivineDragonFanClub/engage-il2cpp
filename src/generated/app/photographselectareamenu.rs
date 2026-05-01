
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectareamenu/PhotographSelectAreaMenu_MenuItem.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectAreaMenu.MenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct PhotographSelectAreaMenu_MenuItem {
    #[rename(name = "m_AreaData")]
    pub m_area_data: crate::app::photographspotdata::PhotographSpotData,
    #[rename(name = "m_SelectHandler")]
    pub m_select_handler:
        crate::app::photographselectareamenu::PhotographSelectAreaMenu_SelectHandler,
}

#[cfg(feature = "app-photographselectareamenu")]
#[::unity2::methods]
impl PhotographSelectAreaMenu_MenuItem {
    #[method(name = "get_AreaData", args = 0)]
    pub fn get_area_data(self) -> crate::app::photographspotdata::PhotographSpotData;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        area_data: crate::app::photographspotdata::PhotographSpotData,
        select_handler : crate :: app :: photographselectareamenu :: PhotographSelectAreaMenu_SelectHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-photographselectareamenu")]
impl PhotographSelectAreaMenu_MenuItem {
    pub fn new(
        area_data: crate::app::photographspotdata::PhotographSpotData,
        select_handler : crate :: app :: photographselectareamenu :: PhotographSelectAreaMenu_SelectHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectAreaMenu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectAreaMenu_MenuItemMethods>::ctor(this, area_data, select_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectareamenu/PhotographSelectAreaMenu.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectAreaMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct PhotographSelectAreaMenu {}

#[cfg(feature = "app-photographselectareamenu")]
#[::unity2::methods]
impl PhotographSelectAreaMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        select_handler : crate :: app :: photographselectareamenu :: PhotographSelectAreaMenu_SelectHandler,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-photographselectareamenu")]
impl PhotographSelectAreaMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectAreaMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectAreaMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectareamenu/PhotographSelectAreaMenu_SelectHandler.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectAreaMenu.SelectHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographSelectAreaMenu_SelectHandler {}

#[cfg(feature = "app-photographselectareamenu")]
#[::unity2::methods]
impl PhotographSelectAreaMenu_SelectHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, area_data: crate::app::photographspotdata::PhotographSpotData) -> ();
}

#[cfg(feature = "app-photographselectareamenu")]
impl PhotographSelectAreaMenu_SelectHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectAreaMenu_SelectHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectAreaMenu_SelectHandlerMethods>::ctor(this, object, method);
        this
    }
}
