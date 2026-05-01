
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/giftmenuitemsub/GiftMenuItemSub.md")))]
#[::unity2::class(namespace = "App", name = "GiftMenuItemSub")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GiftMenuItemSub {
    #[rename(name = "m_Item")]
    pub m_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Value")]
    pub m_value: i32,
}

#[cfg(feature = "app-giftmenuitemsub")]
#[::unity2::methods]
impl GiftMenuItemSub {
    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "set_IsActive", args = 1)]
    pub fn set_is_active(self, value: bool) -> ();

    #[method(name = "get_IsActiveSelect", args = 0)]
    pub fn get_is_active_select(self) -> bool;

    #[method(name = "set_IsActiveSelect", args = 1)]
    pub fn set_is_active_select(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, item: crate::app::itemdata::ItemData, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCountText", args = 0)]
    pub fn get_count_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-giftmenuitemsub")]
impl GiftMenuItemSub {
    pub fn new(item: crate::app::itemdata::ItemData, unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GiftMenuItemSub),
                ::core::stringify!(new),
            )
        });
        <Self as IGiftMenuItemSubMethods>::ctor(this, item, unit);
        this
    }
}
