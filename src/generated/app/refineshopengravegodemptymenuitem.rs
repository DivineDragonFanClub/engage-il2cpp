
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodemptymenuitem/RefineShopEngraveGodEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodEmptyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopEngraveGodEmptyMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu_SelectEventHandler,
}

#[cfg(feature = "app-refineshopengravegodemptymenuitem")]
#[::unity2::methods]
impl RefineShopEngraveGodEmptyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        select_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshopengravegodemptymenuitem")]
impl RefineShopEngraveGodEmptyMenuItem {
    pub fn new(
        select_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodEmptyMenuItemMethods>::ctor(this, select_event_handler);
        this
    }
}
