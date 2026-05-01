
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiiboaccessorymenuitem/AmiiboAccessoryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboAccessoryMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AmiiboAccessoryMenuItem {}

#[cfg(feature = "app-amiiboaccessorymenuitem")]
#[::unity2::methods]
impl AmiiboAccessoryMenuItem {
    #[method(name = "get_AccessoryData", args = 0)]
    pub fn get_accessory_data(self) -> crate::app::accessorydata::AccessoryData;

    #[method(name = "set_AccessoryData", args = 1)]
    pub fn set_accessory_data(self, value: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "IsNew", args = 0)]
    pub fn is_new(self) -> bool;

    #[method(name = "SawNewAccess", args = 0)]
    pub fn saw_new_access(self) -> ();
}

#[cfg(feature = "app-amiiboaccessorymenuitem")]
impl AmiiboAccessoryMenuItem {
    pub fn new(data: crate::app::accessorydata::AccessoryData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboAccessoryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboAccessoryMenuItemMethods>::ctor(this, data);
        this
    }
}
