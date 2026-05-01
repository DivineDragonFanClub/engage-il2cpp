
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemmenuitem/UnitItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct UnitItemMenuItem {
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
    #[rename(name = "m_SelectableBlank")]
    pub m_selectable_blank: bool,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::unititemmenu::UnitItemMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-unititemmenuitem")]
#[::unity2::methods]
impl UnitItemMenuItem {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        i: i32,
        selectable_blank: bool,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetRecieverUnit", args = 0)]
    pub fn get_reciever_unit(self) -> crate::app::unit::Unit;

    #[method(name = "IsVisibleItemIconOnBlank", args = 0)]
    pub fn is_visible_item_icon_on_blank(self) -> bool;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-unititemmenuitem")]
impl UnitItemMenuItem {
    pub fn new(
        i: i32,
        selectable_blank: bool,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemMenuItemMethods>::ctor(
            this,
            i,
            selectable_blank,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}
