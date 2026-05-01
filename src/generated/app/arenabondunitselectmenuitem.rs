
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectmenuitem/ArenaBondUnitSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ArenaBondUnitSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondunitselectmenu::ArenaBondUnitSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::arenabondunitselectmenu::ArenaBondUnitSelectMenu_SelectEventHandler,
    #[rename(name = "IsSelectableUnit")]
    pub is_selectable_unit: bool,
}

#[cfg(feature = "app-arenabondunitselectmenuitem")]
#[::unity2::methods]
impl ArenaBondUnitSelectMenuItem {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_IsSelectable", args = 0)]
    pub fn get_is_selectable(self) -> bool;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-arenabondunitselectmenuitem")]
impl ArenaBondUnitSelectMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectMenuItemMethods>::ctor(
            this,
            unit,
            decide_event_handler,
            select_event_handler,
        );
        this
    }
}
