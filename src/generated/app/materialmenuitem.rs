
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materialmenuitem/MaterialMenuItem_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MaterialMenuItem.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MaterialMenuItem_DecideEventHandler {}

#[cfg(feature = "app-materialmenuitem")]
#[::unity2::methods]
impl MaterialMenuItem_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, material_id: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-materialmenuitem")]
impl MaterialMenuItem_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialMenuItem_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialMenuItem_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materialmenuitem/MaterialMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MaterialMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MaterialMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
}

#[cfg(feature = "app-materialmenuitem")]
#[::unity2::methods]
impl MaterialMenuItem {
    #[method(name = "get_m_ItemRefineExchangeData", args = 0)]
    pub fn get_m_item_refine_exchange_data(
        self,
    ) -> crate::app::itemrefineexchangedata::ItemRefineExchangeData;

    #[method(name = "set_m_ItemRefineExchangeData", args = 1)]
    pub fn set_m_item_refine_exchange_data(
        self,
        value: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetSelectEventHandler", args = 1)]
    pub fn set_select_event_handler(
        self,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-materialmenuitem")]
impl MaterialMenuItem {
    pub fn new(
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialMenuItemMethods>::ctor(
            this,
            item_refine_exchange_data,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materialmenuitem/MaterialMenuItem_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MaterialMenuItem.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MaterialMenuItem_SelectEventHandler {}

#[cfg(feature = "app-materialmenuitem")]
#[::unity2::methods]
impl MaterialMenuItem_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, material_id: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-materialmenuitem")]
impl MaterialMenuItem_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialMenuItem_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialMenuItem_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
