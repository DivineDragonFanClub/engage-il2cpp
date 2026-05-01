
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapteleportmenu/GmapTeleportMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "GmapTeleportMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GmapTeleportMenu_DecideEventHandler {}

#[cfg(feature = "app-gmapteleportmenu")]
#[::unity2::methods]
impl GmapTeleportMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, gmap_spot: crate::app::gmapspot::GmapSpot) -> ();
}

#[cfg(feature = "app-gmapteleportmenu")]
impl GmapTeleportMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapTeleportMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapTeleportMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapteleportmenu/GmapTeleportMenu_GmapTeleportMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "GmapTeleportMenu.GmapTeleportMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GmapTeleportMenu_GmapTeleportMenuItem {
    #[rename(name = "m_GmapSpot")]
    pub m_gmap_spot: crate::app::gmapspot::GmapSpot,
    #[rename(name = "m_SpotPosition")]
    pub m_spot_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CloseMapAndBarFunc")]
    pub m_close_map_and_bar_func: crate::system::action::Action,
    #[rename(name = "m_IsCameraFocus")]
    pub m_is_camera_focus: bool,
}

#[cfg(feature = "app-gmapteleportmenu")]
#[::unity2::methods]
impl GmapTeleportMenu_GmapTeleportMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        gmap_spot: crate::app::gmapspot::GmapSpot,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Spot", args = 0)]
    pub fn get_spot(self) -> crate::app::gmapspot::GmapSpot;
}

#[cfg(feature = "app-gmapteleportmenu")]
impl GmapTeleportMenu_GmapTeleportMenuItem {
    pub fn new(
        gmap_spot: crate::app::gmapspot::GmapSpot,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapTeleportMenu_GmapTeleportMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapTeleportMenu_GmapTeleportMenuItemMethods>::ctor(
            this,
            gmap_spot,
            close_map_and_bar_func,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapteleportmenu/GmapTeleportMenu.md")))]
#[::unity2::class(namespace = "App", name = "GmapTeleportMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GmapTeleportMenu {
    #[rename(name = "m_Content")]
    pub m_content: crate::app::systemscrollmenucontent::SystemScrollMenuContent,
    #[rename(name = "m_GmapInfo")]
    pub m_gmap_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::app::gmapteleportmenu::GmapTeleportMenu_DecideEventHandler,
    #[rename(name = "m_GoToSolanelCallback")]
    pub m_go_to_solanel_callback: crate::system::action::Action,
    #[rename(name = "m_CloseMapAndBarFunc")]
    pub m_close_map_and_bar_func: crate::system::action::Action,
}

#[cfg(feature = "app-gmapteleportmenu")]
#[::unity2::methods]
impl GmapTeleportMenu {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        gmap_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
        decide_event_handler: crate::app::gmapteleportmenu::GmapTeleportMenu_DecideEventHandler,
        goto_solanel_callback: crate::system::action::Action,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> ();

    #[method(name = "AddMenuItems", args = 2)]
    pub fn add_menu_items(
        list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        gmap_map_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
        decide_event_handler: crate::app::gmapteleportmenu::GmapTeleportMenu_DecideEventHandler,
        goto_solanel_callback: crate::system::action::Action,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMenuContent", args = 0)]
    pub fn get_menu_content(self) -> crate::app::systemscrollmenucontent::SystemScrollMenuContent;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-gmapteleportmenu")]
impl GmapTeleportMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        gmap_map_info: crate::app::gmapmapinfocontent::GmapMapInfoContent,
        decide_event_handler: crate::app::gmapteleportmenu::GmapTeleportMenu_DecideEventHandler,
        goto_solanel_callback: crate::system::action::Action,
        close_map_and_bar_func: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapTeleportMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapTeleportMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            gmap_map_info,
            decide_event_handler,
            goto_solanel_callback,
            close_map_and_bar_func,
        );
        this
    }
}
