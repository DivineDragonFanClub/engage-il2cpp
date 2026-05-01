
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringmenuitem/RingMenuItem_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RingMenuItem.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RingMenuItem_SelectEventHandler {}

#[cfg(feature = "app-ringmenuitem")]
#[::unity2::methods]
impl RingMenuItem_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, ring_data: crate::app::ringdata::RingData) -> ();
}

#[cfg(feature = "app-ringmenuitem")]
impl RingMenuItem_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingMenuItem_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRingMenuItem_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringmenuitem/RingMenuItem_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RingMenuItem.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RingMenuItem_DecideEventHandler {}

#[cfg(feature = "app-ringmenuitem")]
#[::unity2::methods]
impl RingMenuItem_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 5)]
    pub fn invoke(
        self,
        ring_data: crate::app::ringdata::RingData,
        base_ring_count: i32,
        piece_of_bonds_count: i32,
        god_unit_index: i32,
        menu_select: crate::app::basicmenuselect::BasicMenuSelect,
    ) -> ();
}

#[cfg(feature = "app-ringmenuitem")]
impl RingMenuItem_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingMenuItem_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRingMenuItem_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringmenuitem/RingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RingMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
}

#[cfg(feature = "app-ringmenuitem")]
#[::unity2::methods]
impl RingMenuItem {
    #[method(name = "get_m_RingData", args = 0)]
    pub fn get_m_ring_data(self) -> crate::app::ringdata::RingData;

    #[method(name = "set_m_RingData", args = 1)]
    pub fn set_m_ring_data(self, value: crate::app::ringdata::RingData) -> ();

    #[method(name = "get_m_IsEquipped", args = 0)]
    pub fn get_m_is_equipped(self) -> bool;

    #[method(name = "set_m_IsEquipped", args = 1)]
    pub fn set_m_is_equipped(self, value: bool) -> ();

    #[method(name = "get_m_Count", args = 0)]
    pub fn get_m_count(self) -> i32;

    #[method(name = "set_m_Count", args = 1)]
    pub fn set_m_count(self, value: i32) -> ();

    #[method(name = "get_m_NeededCountToNext", args = 0)]
    pub fn get_m_needed_count_to_next(self) -> i32;

    #[method(name = "set_m_NeededCountToNext", args = 1)]
    pub fn set_m_needed_count_to_next(self, value: i32) -> ();

    #[method(name = "get_m_NeededPieceOfBondsToNext", args = 0)]
    pub fn get_m_needed_piece_of_bonds_to_next(self) -> i32;

    #[method(name = "set_m_NeededPieceOfBondsToNext", args = 1)]
    pub fn set_m_needed_piece_of_bonds_to_next(self, value: i32) -> ();

    #[method(name = "get_m_IsEnoughCount", args = 0)]
    pub fn get_m_is_enough_count(self) -> bool;

    #[method(name = "set_m_IsEnoughCount", args = 1)]
    pub fn set_m_is_enough_count(self, value: bool) -> ();

    #[method(name = "get_m_IsEnoughPieceOfBond", args = 0)]
    pub fn get_m_is_enough_piece_of_bond(self) -> bool;

    #[method(name = "set_m_IsEnoughPieceOfBond", args = 1)]
    pub fn set_m_is_enough_piece_of_bond(self, value: bool) -> ();

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        ring_data: crate::app::ringdata::RingData,
        is_equipped: bool,
        count: i32,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-ringmenuitem")]
impl RingMenuItem {
    pub fn new(
        ring_data: crate::app::ringdata::RingData,
        is_equipped: bool,
        count: i32,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::ringmenuitem::RingMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRingMenuItemMethods>::ctor(
            this,
            ring_data,
            is_equipped,
            count,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
