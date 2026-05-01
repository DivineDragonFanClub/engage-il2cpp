
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenuitem/RefreshFacilitySelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefreshFacilitySelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefreshFacilitySelectMenuItem {
    #[rename(name = "m_FacilityData")]
    pub m_facility_data: crate::app::hubfacilitydata::HubFacilityData,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-refreshfacilityselectmenuitem")]
#[::unity2::methods]
impl RefreshFacilitySelectMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        facility_aid: ::unity2::Il2CppString,
        select_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refreshfacilityselectmenuitem")]
impl RefreshFacilitySelectMenuItem {
    pub fn new(
        facility_aid: ::unity2::Il2CppString,
        select_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenuItemMethods>::ctor(
            this,
            facility_aid,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
