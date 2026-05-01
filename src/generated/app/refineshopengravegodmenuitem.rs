
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenuitem/RefineShopEngraveGodMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopEngraveGodMenuItem {
    #[rename(name = "m_EngravedUnitItem")]
    pub m_engraved_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineshopengravegodmenuitem")]
#[::unity2::methods]
impl RefineShopEngraveGodMenuItem {
    #[method(name = "get_m_GodData", args = 0)]
    pub fn get_m_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_m_GodData", args = 1)]
    pub fn set_m_god_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "get_m_BaseUnitItem", args = 0)]
    pub fn get_m_base_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_BaseUnitItem", args = 1)]
    pub fn set_m_base_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_IsEnoughPieceOfBond", args = 0)]
    pub fn get_m_is_enough_piece_of_bond(self) -> bool;

    #[method(name = "set_m_IsEnoughPieceOfBond", args = 1)]
    pub fn set_m_is_enough_piece_of_bond(self, value: bool) -> ();

    #[method(name = "get_m_NeededPieceOfBond", args = 0)]
    pub fn get_m_needed_piece_of_bond(self) -> i32;

    #[method(name = "set_m_NeededPieceOfBond", args = 1)]
    pub fn set_m_needed_piece_of_bond(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        base_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
        select_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_DecideEventHandler,
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

#[cfg(feature = "app-refineshopengravegodmenuitem")]
impl RefineShopEngraveGodMenuItem {
    pub fn new(
        base_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
        select_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenuItemMethods>::ctor(
            this,
            base_unit_item,
            god_data,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
