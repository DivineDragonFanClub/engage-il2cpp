
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenuitem/RefreshUnitSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefreshUnitSelectMenuItem {
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: bool,
    #[rename(name = "m_Decided")]
    pub m_decided: bool,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refreshunitselectmenu::RefreshUnitSelectMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshunitselectmenu::RefreshUnitSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-refreshunitselectmenuitem")]
#[::unity2::methods]
impl RefreshUnitSelectMenuItem {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        initial_selected: bool,
        decided: bool,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "UpdateFixedCursor", args = 0)]
    pub fn update_fixed_cursor(self) -> ();

    #[method(name = "SetDecided", args = 1)]
    pub fn set_decided(self, decided: bool) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refreshunitselectmenuitem")]
impl RefreshUnitSelectMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        initial_selected: bool,
        decided: bool,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenuItemMethods>::ctor(
            this,
            unit,
            initial_selected,
            decided,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
