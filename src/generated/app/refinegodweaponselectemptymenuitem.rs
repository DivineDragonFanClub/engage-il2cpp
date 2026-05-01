
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::refinegodweaponselectmenuitem::IRefineGodWeaponSelectMenuItem;
use crate::app::refinegodweaponselectmenuitem::RefineGodWeaponSelectMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponselectemptymenuitem/RefineGodWeaponSelectEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponSelectEmptyMenuItem")]
#[parent(crate::app::refinegodweaponselectmenuitem::RefineGodWeaponSelectMenuItem)]
pub struct RefineGodWeaponSelectEmptyMenuItem {}

#[cfg(feature = "app-refinegodweaponselectemptymenuitem")]
#[::unity2::methods]
impl RefineGodWeaponSelectEmptyMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        select_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_SelectEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refinegodweaponselectemptymenuitem")]
impl RefineGodWeaponSelectEmptyMenuItem {
    pub fn new(
        select_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_SelectEventHandler,
        request_close_event_handler : crate :: app :: refinegodweaponselectmenu :: RefineGodWeaponSelectMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponSelectEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponSelectEmptyMenuItemMethods>::ctor(
            this,
            select_event_handler,
            request_close_event_handler,
        );
        this
    }
}
