
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::stringitem::IStringItem;
use crate::app::stringitem::StringItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/labelitem/LabelItem.md")))]
#[::unity2::class(namespace = "App", name = "LabelItem")]
#[parent(crate::app::stringitem::StringItem)]
pub struct LabelItem {
    #[static_field]
    #[rename(name = "COLOR_BACK")]
    pub color_back: crate::unity_engine::color::Color,
    #[static_field]
    #[rename(name = "COLOR_FONT")]
    pub color_font: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-labelitem")]
#[::unity2::methods]
impl LabelItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString, english: ::unity2::Il2CppString) -> ();

    #[method(name = "IsSelectable", args = 0)]
    pub fn is_selectable(self) -> bool;

    #[method(name = "GetBackColor", args = 0)]
    pub fn get_back_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetFontColor", args = 0)]
    pub fn get_font_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetMarginWidth", args = 0)]
    pub fn get_margin_width(self) -> f32;

    #[method(name = "GetMarginHeight", args = 0)]
    pub fn get_margin_height(self) -> f32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-labelitem")]
impl LabelItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LabelItem),
                ::core::stringify!(new),
            )
        });
        <Self as ILabelItemMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString, english: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LabelItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as ILabelItemMethods>::ctor_2(this, name, english);
        this
    }
}
