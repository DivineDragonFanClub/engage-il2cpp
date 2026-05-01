
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::materialmenuitem::IMaterialMenuItem;
use crate::app::materialmenuitem::MaterialMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangetargetmenuitem/RefineShopExchangeTargetMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeTargetMenuItem")]
#[parent(crate::app::materialmenuitem::MaterialMenuItem)]
pub struct RefineShopExchangeTargetMenuItem {}

#[cfg(feature = "app-refineshopexchangetargetmenuitem")]
#[::unity2::methods]
impl RefineShopExchangeTargetMenuItem {
    #[method(name = "get_m_EnoughMaterial", args = 0)]
    pub fn get_m_enough_material(self) -> bool;

    #[method(name = "set_m_EnoughMaterial", args = 1)]
    pub fn set_m_enough_material(self, value: bool) -> ();

    #[method(name = "get_m_MaxMaterial", args = 0)]
    pub fn get_m_max_material(self) -> bool;

    #[method(name = "set_m_MaxMaterial", args = 1)]
    pub fn set_m_max_material(self, value: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-refineshopexchangetargetmenuitem")]
impl RefineShopExchangeTargetMenuItem {
    pub fn new(
        item_refine_exchange_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        select_event_handler: crate::app::materialmenuitem::MaterialMenuItem_SelectEventHandler,
        decide_event_handler: crate::app::materialmenuitem::MaterialMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeTargetMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeTargetMenuItemMethods>::ctor(
            this,
            item_refine_exchange_data,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
