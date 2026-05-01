
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::ringmenuitem::IRingMenuItem;
use crate::app::ringmenuitem::RingMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringemptymenuitem/RingEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RingEmptyMenuItem")]
#[parent(crate::app::ringmenuitem::RingMenuItem)]
pub struct RingEmptyMenuItem {}

#[cfg(feature = "app-ringemptymenuitem")]
#[::unity2::methods]
impl RingEmptyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-ringemptymenuitem")]
impl RingEmptyMenuItem {
    pub fn new(
        select_event_handler: crate::app::ringmenuitem::RingMenuItem_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRingEmptyMenuItemMethods>::ctor(this, select_event_handler);
        this
    }
}
