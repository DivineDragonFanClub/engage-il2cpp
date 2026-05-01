
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetmenuitem/RefreshUnitSetMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSetMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefreshUnitSetMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Decided")]
    pub m_decided: bool,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshunitsetmenu::RefreshUnitSetMenu_DecideEventHandler,
}

#[cfg(feature = "app-refreshunitsetmenuitem")]
#[::unity2::methods]
impl RefreshUnitSetMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        decide_event_handler: crate::app::refreshunitsetmenu::RefreshUnitSetMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateFixedCursor", args = 0)]
    pub fn update_fixed_cursor(self) -> ();

    #[method(name = "SetDecided", args = 1)]
    pub fn set_decided(self, decided: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refreshunitsetmenuitem")]
impl RefreshUnitSetMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        decide_event_handler: crate::app::refreshunitsetmenu::RefreshUnitSetMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetMenuItemMethods>::ctor(this, unit, decide_event_handler);
        this
    }
}
