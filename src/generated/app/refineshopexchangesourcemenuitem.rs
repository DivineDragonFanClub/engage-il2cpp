
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::materialmenuitem::IMaterialMenuItem;
use crate::app::materialmenuitem::MaterialMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangesourcemenuitem/RefineShopExchangeSourceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeSourceMenuItem")]
#[parent(crate::app::materialmenuitem::MaterialMenuItem)]
pub struct RefineShopExchangeSourceMenuItem {}

#[cfg(feature = "app-refineshopexchangesourcemenuitem")]
#[::unity2::methods]
impl RefineShopExchangeSourceMenuItem {
    #[method(name = "get_m_EnoughSourceCount", args = 0)]
    pub fn get_m_enough_source_count(self) -> bool;

    #[method(name = "set_m_EnoughSourceCount", args = 1)]
    pub fn set_m_enough_source_count(self, value: bool) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        target_material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-refineshopexchangesourcemenuitem")]
impl RefineShopExchangeSourceMenuItem {
    pub fn new(
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        target_material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeSourceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeSourceMenuItemMethods>::ctor(
            this,
            item_refine_exchange_data,
            target_material_data,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
