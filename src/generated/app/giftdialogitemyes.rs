
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/giftdialogitemyes/GiftDialogItemYes.md")))]
#[::unity2::class(namespace = "App", name = "GiftDialogItemYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct GiftDialogItemYes {
    #[rename(name = "m_Item")]
    pub m_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Value")]
    pub m_value: i32,
}

#[cfg(feature = "app-giftdialogitemyes")]
#[::unity2::methods]
impl GiftDialogItemYes {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        item: crate::app::itemdata::ItemData,
        unit: crate::app::unit::Unit,
        value: i32,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-giftdialogitemyes")]
impl GiftDialogItemYes {
    pub fn new(
        item: crate::app::itemdata::ItemData,
        unit: crate::app::unit::Unit,
        value: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GiftDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IGiftDialogItemYesMethods>::ctor(this, item, unit, value);
        this
    }
}
