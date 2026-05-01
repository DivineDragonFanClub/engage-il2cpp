
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::unititemmenuitem::IUnitItemMenuItem;
use crate::app::unititemmenuitem::UnitItemMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititememptymenuitem/UnitItemEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemEmptyMenuItem")]
#[parent(crate::app::unititemmenuitem::UnitItemMenuItem)]
pub struct UnitItemEmptyMenuItem {
    #[rename(name = "m_EmptyUnitItem")]
    pub m_empty_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-unititememptymenuitem")]
#[::unity2::methods]
impl UnitItemEmptyMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-unititememptymenuitem")]
impl UnitItemEmptyMenuItem {
    pub fn new(
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemEmptyMenuItemMethods>::ctor(
            this,
            select_event_handler,
            request_close_event_handler,
        );
        this
    }
}
