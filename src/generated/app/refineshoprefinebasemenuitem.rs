
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinebasemenuitem/RefineShopRefineBaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineBaseMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct RefineShopRefineBaseMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refineshoprefinebasemenu::RefineShopRefineBaseMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refineshoprefinebasemenu::RefineShopRefineBaseMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineshoprefinebasemenuitem")]
#[::unity2::methods]
impl RefineShopRefineBaseMenuItem {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_m_ItemIndex", args = 0)]
    pub fn get_m_item_index(self) -> i32;

    #[method(name = "set_m_ItemIndex", args = 1)]
    pub fn set_m_item_index(self, value: i32) -> ();

    #[method(name = "get_m_UnitItem", args = 0)]
    pub fn get_m_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_UnitItem", args = 1)]
    pub fn set_m_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_SortValue", args = 0)]
    pub fn get_m_sort_value(self) -> i64;

    #[method(name = "set_m_SortValue", args = 1)]
    pub fn set_m_sort_value(self, value: i64) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        item_index: i32,
        select_event_handler : crate :: app :: refineshoprefinebasemenu :: RefineShopRefineBaseMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshoprefinebasemenu :: RefineShopRefineBaseMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoprefinebasemenuitem")]
impl RefineShopRefineBaseMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        item_index: i32,
        select_event_handler : crate :: app :: refineshoprefinebasemenu :: RefineShopRefineBaseMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshoprefinebasemenu :: RefineShopRefineBaseMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineBaseMenuItemMethods>::ctor(
            this,
            unit,
            item_index,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
