
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponselectmenuitem/RefineGodWeaponSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineGodWeaponSelectMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refinegodweaponselectmenu::RefineGodWeaponSelectMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refinegodweaponselectmenu::RefineGodWeaponSelectMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::refinegodweaponselectmenu::RefineGodWeaponSelectMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-refinegodweaponselectmenuitem")]
#[::unity2::methods]
impl RefineGodWeaponSelectMenuItem {
    #[method(name = "get_m_ItemData", args = 0)]
    pub fn get_m_item_data(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_m_ItemData", args = 1)]
    pub fn set_m_item_data(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_m_UnitItem", args = 0)]
    pub fn get_m_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_UnitItem", args = 1)]
    pub fn set_m_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_Capacity", args = 0)]
    pub fn get_m_capacity(self) -> i32;

    #[method(name = "set_m_Capacity", args = 1)]
    pub fn set_m_capacity(self, value: i32) -> ();

    #[method(name = "get_m_CapacityMax", args = 0)]
    pub fn get_m_capacity_max(self) -> i32;

    #[method(name = "set_m_CapacityMax", args = 1)]
    pub fn set_m_capacity_max(self, value: i32) -> ();

    #[method(name = "get_m_Empty", args = 0)]
    pub fn get_m_empty(self) -> bool;

    #[method(name = "set_m_Empty", args = 1)]
    pub fn set_m_empty(self, value: bool) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        god_weapon_data: crate::app::itemdata::ItemData,
        using_capacity: i32,
        capacity: i32,
        select_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_RequestCloseEventHandler,
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

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refinegodweaponselectmenuitem")]
impl RefineGodWeaponSelectMenuItem {
    pub fn new(
        god_weapon_data: crate::app::itemdata::ItemData,
        using_capacity: i32,
        capacity: i32,
        select_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponSelectMenuItemMethods>::ctor(
            this,
            god_weapon_data,
            using_capacity,
            capacity,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}
