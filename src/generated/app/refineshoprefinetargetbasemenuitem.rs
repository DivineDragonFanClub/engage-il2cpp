
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetbasemenuitem/RefineShopRefineTargetBaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineTargetBaseMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopRefineTargetBaseMenuItem {
    #[rename(name = "m_BaseUnitItem")]
    pub m_base_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-refineshoprefinetargetbasemenuitem")]
#[::unity2::methods]
impl RefineShopRefineTargetBaseMenuItem {
    #[method(name = "get_m_TargetUnitItem", args = 0)]
    pub fn get_m_target_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_TargetUnitItem", args = 1)]
    pub fn set_m_target_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_NeededIron", args = 0)]
    pub fn get_m_needed_iron(self) -> i32;

    #[method(name = "set_m_NeededIron", args = 1)]
    pub fn set_m_needed_iron(self, value: i32) -> ();

    #[method(name = "get_m_NeededSteel", args = 0)]
    pub fn get_m_needed_steel(self) -> i32;

    #[method(name = "set_m_NeededSteel", args = 1)]
    pub fn set_m_needed_steel(self, value: i32) -> ();

    #[method(name = "get_m_NeededSilver", args = 0)]
    pub fn get_m_needed_silver(self) -> i32;

    #[method(name = "set_m_NeededSilver", args = 1)]
    pub fn set_m_needed_silver(self, value: i32) -> ();

    #[method(name = "get_m_NeededMoney", args = 0)]
    pub fn get_m_needed_money(self) -> i32;

    #[method(name = "set_m_NeededMoney", args = 1)]
    pub fn set_m_needed_money(self, value: i32) -> ();

    #[method(name = "get_m_IsEnoughIron", args = 0)]
    pub fn get_m_is_enough_iron(self) -> bool;

    #[method(name = "set_m_IsEnoughIron", args = 1)]
    pub fn set_m_is_enough_iron(self, value: bool) -> ();

    #[method(name = "get_m_IsEnoughSteel", args = 0)]
    pub fn get_m_is_enough_steel(self) -> bool;

    #[method(name = "set_m_IsEnoughSteel", args = 1)]
    pub fn set_m_is_enough_steel(self, value: bool) -> ();

    #[method(name = "get_m_IsEnoughSilver", args = 0)]
    pub fn get_m_is_enough_silver(self) -> bool;

    #[method(name = "set_m_IsEnoughSilver", args = 1)]
    pub fn set_m_is_enough_silver(self, value: bool) -> ();

    #[method(name = "get_m_IsEnoughMoney", args = 0)]
    pub fn get_m_is_enough_money(self) -> bool;

    #[method(name = "set_m_IsEnoughMoney", args = 1)]
    pub fn set_m_is_enough_money(self, value: bool) -> ();

    #[method(name = "get_m_IsNameVisible", args = 0)]
    pub fn get_m_is_name_visible(self) -> bool;

    #[method(name = "set_m_IsNameVisible", args = 1)]
    pub fn set_m_is_name_visible(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

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
}

#[cfg(feature = "app-refineshoprefinetargetbasemenuitem")]
impl RefineShopRefineTargetBaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetBaseMenuItemMethods>::ctor(this);
        this
    }
}
