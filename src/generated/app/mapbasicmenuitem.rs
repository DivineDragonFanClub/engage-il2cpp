
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbasicmenuitem/MapBasicMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapBasicMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MapBasicMenuItem {}

#[cfg(feature = "app-mapbasicmenuitem")]
#[::unity2::methods]
impl MapBasicMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbasicmenuitem")]
impl MapBasicMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBasicMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBasicMenuItemMethods>::ctor(this);
        this
    }
}
