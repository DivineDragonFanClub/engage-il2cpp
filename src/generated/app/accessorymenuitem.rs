
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorymenuitem/AccessoryMenuItem_RequestCloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryMenuItem.RequestCloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryMenuItem_RequestCloseEventHandler {}

#[cfg(feature = "app-accessorymenuitem")]
#[::unity2::methods]
impl AccessoryMenuItem_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessorymenuitem")]
impl AccessoryMenuItem_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryMenuItem_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryMenuItem_RequestCloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorymenuitem/AccessoryMenuItem_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryMenuItem.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryMenuItem_DecideEventHandler {}

#[cfg(feature = "app-accessorymenuitem")]
#[::unity2::methods]
impl AccessoryMenuItem_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessorymenuitem")]
impl AccessoryMenuItem_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryMenuItem_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryMenuItem_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorymenuitem/AccessoryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AccessoryMenuItem {
    #[rename(name = "m_AlwaysActive")]
    pub m_always_active: bool,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
}

#[cfg(feature = "app-accessorymenuitem")]
#[::unity2::methods]
impl AccessoryMenuItem {
    #[method(name = "get_m_AccessoryData", args = 0)]
    pub fn get_m_accessory_data(self) -> crate::app::accessorydata::AccessoryData;

    #[method(name = "set_m_AccessoryData", args = 1)]
    pub fn set_m_accessory_data(self, value: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "get_m_AccessoryKind", args = 0)]
    pub fn get_m_accessory_kind(self) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "set_m_AccessoryKind", args = 1)]
    pub fn set_m_accessory_kind(self, value: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = "get_m_Decided", args = 0)]
    pub fn get_m_decided(self) -> bool;

    #[method(name = "set_m_Decided", args = 1)]
    pub fn set_m_decided(self, value: bool) -> ();

    #[method(name = "get_m_Female", args = 0)]
    pub fn get_m_female(self) -> crate::app::accessoryshoputility::AccessoryShopUtility_Female;

    #[method(name = "set_m_Female", args = 1)]
    pub fn set_m_female(
        self,
        value: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
    ) -> ();

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        accessory_data: crate::app::accessorydata::AccessoryData,
        accessory_kind: crate::app::accessorydata::AccessoryData_Kinds,
        decided: bool,
        always_active: bool,
        female: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetData", args = 3)]
    pub fn set_data(
        self,
        accessory_data: crate::app::accessorydata::AccessoryData,
        accessory_kind: crate::app::accessorydata::AccessoryData_Kinds,
        female: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
    ) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "GetRectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "SetDecide", args = 0)]
    pub fn set_decide(self) -> ();

    #[method(name = "UnsetDecide", args = 0)]
    pub fn unset_decide(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-accessorymenuitem")]
impl AccessoryMenuItem {
    pub fn new(
        accessory_data: crate::app::accessorydata::AccessoryData,
        accessory_kind: crate::app::accessorydata::AccessoryData_Kinds,
        decided: bool,
        always_active: bool,
        female: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
        select_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::accessorymenuitem::AccessoryMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryMenuItemMethods>::ctor(
            this,
            accessory_data,
            accessory_kind,
            decided,
            always_active,
            female,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorymenuitem/AccessoryMenuItem_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryMenuItem.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryMenuItem_SelectEventHandler {}

#[cfg(feature = "app-accessorymenuitem")]
#[::unity2::methods]
impl AccessoryMenuItem_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();
}

#[cfg(feature = "app-accessorymenuitem")]
impl AccessoryMenuItem_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryMenuItem_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryMenuItem_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
