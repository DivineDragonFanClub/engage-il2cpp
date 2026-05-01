
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::refineshoprefinetargetbasemenuitem::IRefineShopRefineTargetBaseMenuItem;
use crate::app::refineshoprefinetargetbasemenuitem::RefineShopRefineTargetBaseMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetevolvemenuitem/RefineShopRefineTargetEvolveMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineTargetEvolveMenuItem")]
#[parent(crate::app::refineshoprefinetargetbasemenuitem::RefineShopRefineTargetBaseMenuItem)]
pub struct RefineShopRefineTargetEvolveMenuItem {
# [rename (name = "m_EvolvedUnitItem")] pub m_evolved_unit_item : crate :: app :: unititem :: UnitItem ,
# [rename (name = "m_EvolveIndex")] pub m_evolve_index : i32 ,
# [rename (name = "m_EvolveData")] pub m_evolve_data : crate :: app :: itemevolvedata :: ItemEvolveData ,
# [rename (name = "m_SelectEventHandler")] pub m_select_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_SelectEventHandler ,
# [rename (name = "m_DecideEventHandler")] pub m_decide_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToEvolveEventHandler ,
}

#[cfg(feature = "app-refineshoprefinetargetevolvemenuitem")]
#[::unity2::methods]
impl RefineShopRefineTargetEvolveMenuItem {
    #[method(name = "get_m_TargetUnitItem", args = 0)]
    pub fn get_m_target_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_TargetUnitItem", args = 1)]
    pub fn set_m_target_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        unit_item: crate::app::unititem::UnitItem,
        evolve_index: i32,
        evolve_data: crate::app::itemevolvedata::ItemEvolveData,
        select_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToEvolveEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoprefinetargetevolvemenuitem")]
impl RefineShopRefineTargetEvolveMenuItem {
    pub fn new(
        unit_item: crate::app::unititem::UnitItem,
        evolve_index: i32,
        evolve_data: crate::app::itemevolvedata::ItemEvolveData,
        select_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToEvolveEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetEvolveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetEvolveMenuItemMethods>::ctor(
            this,
            unit_item,
            evolve_index,
            evolve_data,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
