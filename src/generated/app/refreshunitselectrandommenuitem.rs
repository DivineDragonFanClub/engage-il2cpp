
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::refreshunitselectmenuitem::IRefreshUnitSelectMenuItem;
use crate::app::refreshunitselectmenuitem::RefreshUnitSelectMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectrandommenuitem/RefreshUnitSelectRandomMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectRandomMenuItem")]
#[parent(crate::app::refreshunitselectmenuitem::RefreshUnitSelectMenuItem)]
pub struct RefreshUnitSelectRandomMenuItem {}

#[cfg(feature = "app-refreshunitselectrandommenuitem")]
#[::unity2::methods]
impl RefreshUnitSelectRandomMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        decided: bool,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refreshunitselectrandommenuitem")]
impl RefreshUnitSelectRandomMenuItem {
    pub fn new(
        decided: bool,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectRandomMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectRandomMenuItemMethods>::ctor(
            this,
            decided,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
