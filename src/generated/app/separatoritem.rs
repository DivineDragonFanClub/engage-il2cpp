
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/separatoritem/SeparatorItem.md")))]
#[::unity2::class(namespace = "App", name = "SeparatorItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct SeparatorItem {
    #[static_field]
    #[rename(name = "BackColor")]
    pub back_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-separatoritem")]
#[::unity2::methods]
impl SeparatorItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "GetBackColor", args = 0)]
    pub fn get_back_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "IsSelectable", args = 0)]
    pub fn is_selectable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-separatoritem")]
impl SeparatorItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SeparatorItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISeparatorItemMethods>::ctor(this);
        this
    }
}
