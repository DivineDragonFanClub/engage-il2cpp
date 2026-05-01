
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopunitselectmenuitem/ShopUnitSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopUnitSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ShopUnitSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::shopunitselectmenu::ShopUnitSelectMenu_SelectEventHandler,
}

#[cfg(feature = "app-shopunitselectmenuitem")]
#[::unity2::methods]
impl ShopUnitSelectMenuItem {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        decide_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
        select_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-shopunitselectmenuitem")]
impl ShopUnitSelectMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        decide_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
        select_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopUnitSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopUnitSelectMenuItemMethods>::ctor(
            this,
            unit,
            decide_event_handler,
            select_event_handler,
        );
        this
    }
}
