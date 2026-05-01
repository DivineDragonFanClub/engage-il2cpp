
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammenuitem/RefineGodWeaponParamMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponParamMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineGodWeaponParamMenuItem {
    #[rename(name = "m_RefineOrReset")]
    pub m_refine_or_reset: bool,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-refinegodweaponparammenuitem")]
#[::unity2::methods]
impl RefineGodWeaponParamMenuItem {
    #[method(name = "get_m_GodUnit", args = 0)]
    pub fn get_m_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_m_GodUnit", args = 1)]
    pub fn set_m_god_unit(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_m_GodWeaponRefineData", args = 0)]
    pub fn get_m_god_weapon_refine_data(
        self,
    ) -> crate::app::godweaponrefinedata::GodWeaponRefineData;

    #[method(name = "set_m_GodWeaponRefineData", args = 1)]
    pub fn set_m_god_weapon_refine_data(
        self,
        value: crate::app::godweaponrefinedata::GodWeaponRefineData,
    ) -> ();

    #[method(name = "get_m_RefineKind", args = 0)]
    pub fn get_m_refine_kind(self) -> crate::app::godweaponrefinedata::GodWeaponRefineData_Kind;

    #[method(name = "set_m_RefineKind", args = 1)]
    pub fn set_m_refine_kind(
        self,
        value: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ();

    #[method(name = "get_m_RefineLevel", args = 0)]
    pub fn get_m_refine_level(self) -> i32;

    #[method(name = "set_m_RefineLevel", args = 1)]
    pub fn set_m_refine_level(self, value: i32) -> ();

    #[method(name = "get_m_RefineLevelNext", args = 0)]
    pub fn get_m_refine_level_next(self) -> i32;

    #[method(name = "set_m_RefineLevelNext", args = 1)]
    pub fn set_m_refine_level_next(self, value: i32) -> ();

    #[method(name = "get_m_HasNextLevel", args = 0)]
    pub fn get_m_has_next_level(self) -> bool;

    #[method(name = "set_m_HasNextLevel", args = 1)]
    pub fn set_m_has_next_level(self, value: bool) -> ();

    #[method(name = "get_m_Capacity", args = 0)]
    pub fn get_m_capacity(self) -> i32;

    #[method(name = "set_m_Capacity", args = 1)]
    pub fn set_m_capacity(self, value: i32) -> ();

    #[method(name = "get_m_Material", args = 0)]
    pub fn get_m_material(self) -> i32;

    #[method(name = "set_m_Material", args = 1)]
    pub fn set_m_material(self, value: i32) -> ();

    #[method(name = "get_m_IsSkill", args = 0)]
    pub fn get_m_is_skill(self) -> bool;

    #[method(name = "set_m_IsSkill", args = 1)]
    pub fn set_m_is_skill(self, value: bool) -> ();

    #[method(name = "get_m_EnoughCapacity", args = 0)]
    pub fn get_m_enough_capacity(self) -> bool;

    #[method(name = "set_m_EnoughCapacity", args = 1)]
    pub fn set_m_enough_capacity(self, value: bool) -> ();

    #[method(name = "get_m_EnoughMaterial", args = 0)]
    pub fn get_m_enough_material(self) -> bool;

    #[method(name = "set_m_EnoughMaterial", args = 1)]
    pub fn set_m_enough_material(self, value: bool) -> ();

    #[method(name = "get_m_Empty", args = 0)]
    pub fn get_m_empty(self) -> bool;

    #[method(name = "set_m_Empty", args = 1)]
    pub fn set_m_empty(self, value: bool) -> ();

    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        god_weapon_refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_level: i32,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
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

#[cfg(feature = "app-refinegodweaponparammenuitem")]
impl RefineGodWeaponParamMenuItem {
    pub fn new(
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        god_weapon_refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_level: i32,
        refine_or_reset: bool,
        select_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponparammenu :: RefineGodWeaponParamMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamMenuItemMethods>::ctor(
            this,
            god_unit,
            god_weapon,
            god_weapon_refine_data,
            refine_kind,
            refine_level,
            refine_or_reset,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}
